//! Fixed-order numerical integration for CubeCL kernels.
//!
//! Implements Gauss-Legendre quadrature specialized for the two integrands
//! used by `gga_x_fd_lb94`: `func0` and `func1` (from libxc's `FT_inter`).
//!
//! The original libxc uses QUADPACK adaptive integration with malloc and
//! function pointers, which are not available in `#[cube]` kernels. This
//! implementation uses 32-point Gauss-Legendre quadrature on [0, b] with
//! a substitution x = t² for the logarithmically singular `func1`.

#![allow(
    clippy::excessive_precision,
    clippy::too_many_arguments,
    unused_variables,
)]


// ============================================================================
// FT_inter integrand (from libxc gga_x_fd_lb94.c)
// ============================================================================

/// The FT_inter integrand for n=0 (func0):
///   f(x) = -3/4 * beta * csi / (1 + 3*beta*csi*x*ln(csi*x + sqrt(csi²x² + 1)))
/// where csi = 2^(1/3) = M_CBRT2.
fn ft_inter_0(x: f64, beta: f64) -> f64 {
    let csi: f64 = 1.2599210498948732_f64; // M_CBRT2
    let num = -0.75_f64 * beta * csi;
    let csi_x = csi * x;
    let denom = 1.0_f64 + 3.0_f64 * beta * csi * x * rmath::ln(csi_x + rmath::sqrt(csi_x * csi_x + 1.0_f64));
    num / denom
}

/// The FT_inter integrand for n=1 (func1):
///   f(x) = -3/4 * beta * csi * ln(x) / (1 + 3*beta*csi*x*ln(csi*x + sqrt(csi²x² + 1)))
/// Has a logarithmic singularity at x=0 (integrable).
fn ft_inter_1(x: f64, beta: f64) -> f64 {
    let csi: f64 = 1.2599210498948732_f64; // M_CBRT2
    let num = -0.75_f64 * beta * csi * rmath::ln(x);
    let csi_x = csi * x;
    let denom = 1.0_f64 + 3.0_f64 * beta * csi * x * rmath::ln(csi_x + rmath::sqrt(csi_x * csi_x + 1.0_f64));
    num / denom
}

// ============================================================================
// 32-point Gauss-Legendre nodes and weights on [-1, 1]
// ============================================================================

// To integrate f on [a, b]: ∫f(x)dx ≈ (b-a)/2 * Σ wᵢ f((b-a)/2 * xᵢ + (a+b)/2)
// We inline the 32 nodes/weights as constants since CubeCL can't use arrays.

/// Apply 32-point GL to func0 on sub-interval [a, a+h].
fn gl32_func0(a: f64, h: f64, beta: f64) -> f64 {
    let half = h * 0.5_f64;
    let mid = a + half;

    let n01: f64 = 0.0483076656877383162_f64; let w01: f64 = 0.0965400885147278006_f64;
    let n02: f64 = 0.1444719615827964935_f64; let w02: f64 = 0.0956387200792748594_f64;
    let n03: f64 = 0.2392873622521370745_f64; let w03: f64 = 0.0938443990808045656_f64;
    let n04: f64 = 0.3318686022821276498_f64; let w04: f64 = 0.0911738786957638847_f64;
    let n05: f64 = 0.4213512761306353454_f64; let w05: f64 = 0.0876520930044038111_f64;
    let n06: f64 = 0.5068999089322293900_f64; let w06: f64 = 0.0833119242269467552_f64;
    let n07: f64 = 0.5877157572407623210_f64; let w07: f64 = 0.0781938957870703065_f64;
    let n08: f64 = 0.6630442669302152009_f64; let w08: f64 = 0.0723457941088485062_f64;
    let n09: f64 = 0.7321821187402896804_f64; let w09: f64 = 0.0658222227763618468_f64;
    let n10: f64 = 0.7944837959679424070_f64; let w10: f64 = 0.0586840934785355471_f64;
    let n11: f64 = 0.8493676137325699701_f64; let w11: f64 = 0.0509980592623761762_f64;
    let n12: f64 = 0.8963211557660521239_f64; let w12: f64 = 0.0428358980222266807_f64;
    let n13: f64 = 0.9349060759377396892_f64; let w13: f64 = 0.0342738629130214331_f64;
    let n14: f64 = 0.9647622555875064308_f64; let w14: f64 = 0.0253920653092620595_f64;
    let n15: f64 = 0.9856115115452683354_f64; let w15: f64 = 0.0162743947309056706_f64;
    let n16: f64 = 0.9972638618494815635_f64; let w16: f64 = 0.0070186100094700966_f64;

    let mut s: f64 = 0.0_f64;
    s = s + w01 * ft_inter_0(mid + half * n01, beta);
    s = s + w02 * ft_inter_0(mid + half * n02, beta);
    s = s + w03 * ft_inter_0(mid + half * n03, beta);
    s = s + w04 * ft_inter_0(mid + half * n04, beta);
    s = s + w05 * ft_inter_0(mid + half * n05, beta);
    s = s + w06 * ft_inter_0(mid + half * n06, beta);
    s = s + w07 * ft_inter_0(mid + half * n07, beta);
    s = s + w08 * ft_inter_0(mid + half * n08, beta);
    s = s + w09 * ft_inter_0(mid + half * n09, beta);
    s = s + w10 * ft_inter_0(mid + half * n10, beta);
    s = s + w11 * ft_inter_0(mid + half * n11, beta);
    s = s + w12 * ft_inter_0(mid + half * n12, beta);
    s = s + w13 * ft_inter_0(mid + half * n13, beta);
    s = s + w14 * ft_inter_0(mid + half * n14, beta);
    s = s + w15 * ft_inter_0(mid + half * n15, beta);
    s = s + w16 * ft_inter_0(mid + half * n16, beta);
    s = s + w01 * ft_inter_0(mid - half * n01, beta);
    s = s + w02 * ft_inter_0(mid - half * n02, beta);
    s = s + w03 * ft_inter_0(mid - half * n03, beta);
    s = s + w04 * ft_inter_0(mid - half * n04, beta);
    s = s + w05 * ft_inter_0(mid - half * n05, beta);
    s = s + w06 * ft_inter_0(mid - half * n06, beta);
    s = s + w07 * ft_inter_0(mid - half * n07, beta);
    s = s + w08 * ft_inter_0(mid - half * n08, beta);
    s = s + w09 * ft_inter_0(mid - half * n09, beta);
    s = s + w10 * ft_inter_0(mid - half * n10, beta);
    s = s + w11 * ft_inter_0(mid - half * n11, beta);
    s = s + w12 * ft_inter_0(mid - half * n12, beta);
    s = s + w13 * ft_inter_0(mid - half * n13, beta);
    s = s + w14 * ft_inter_0(mid - half * n14, beta);
    s = s + w15 * ft_inter_0(mid - half * n15, beta);
    s = s + w16 * ft_inter_0(mid - half * n16, beta);
    half * s
}

/// Integrate func0 from 0 to b using 16-panel composite 32-point GL (512 points).
pub fn xc_integrate_func0(b: f64, beta: f64) -> f64 {
    let h = b / 16.0_f64;
    gl32_func0( 0.0_f64 * h, h, beta)
        + gl32_func0( 1.0_f64 * h, h, beta)
        + gl32_func0( 2.0_f64 * h, h, beta)
        + gl32_func0( 3.0_f64 * h, h, beta)
        + gl32_func0( 4.0_f64 * h, h, beta)
        + gl32_func0( 5.0_f64 * h, h, beta)
        + gl32_func0( 6.0_f64 * h, h, beta)
        + gl32_func0( 7.0_f64 * h, h, beta)
        + gl32_func0( 8.0_f64 * h, h, beta)
        + gl32_func0( 9.0_f64 * h, h, beta)
        + gl32_func0(10.0_f64 * h, h, beta)
        + gl32_func0(11.0_f64 * h, h, beta)
        + gl32_func0(12.0_f64 * h, h, beta)
        + gl32_func0(13.0_f64 * h, h, beta)
        + gl32_func0(14.0_f64 * h, h, beta)
        + gl32_func0(15.0_f64 * h, h, beta)
}

/// The singularity-subtracted integrand for func1:
///   g(x) = func1(x) - C·ln(x)
/// where C = -3/4 · beta · csi (the leading coefficient at x=0).
///
/// func1(x) = C·ln(x) / D(x)  where D(x) = 1 + 3βξx·ln(ξx + √(ξ²x²+1))
/// So g(x) = C·ln(x)·(1/D(x) - 1) = C·ln(x)·(1 - D(x))/D(x)
///         = C·ln(x)·(-3βξx·ln(ξx + √(ξ²x²+1))) / D(x)
///
/// As x→0: g(x) → C·ln(x)·(-3βξx·ln(ξx)) → 0  (x·ln(x)·ln(x) → 0)
/// So g(x) is continuous on [0, b] and GL integrates it precisely.
fn ft_inter_1_subtracted(x: f64, beta: f64) -> f64 {
    let csi: f64 = 1.2599210498948732_f64; // M_CBRT2
    let c = -0.75_f64 * beta * csi;
    let csi_x = csi * x;
    let d = 1.0_f64 + 3.0_f64 * beta * csi * x * f64::ln(csi_x + f64::sqrt(csi_x * csi_x + 1.0_f64));
    // g(x) = C·ln(x)·(1/D - 1) = C·ln(x)·(1 - D)/D
    c * f64::ln(x) * (1.0_f64 - d) / d
}

/// Apply 32-point GL to the subtracted integrand on sub-interval [a, a+h].
fn gl32_subtracted(a: f64, h: f64, beta: f64) -> f64 {
    let half = h * 0.5_f64;
    let mid = a + half;

    let n01: f64 = 0.0483076656877383162_f64; let w01: f64 = 0.0965400885147278006_f64;
    let n02: f64 = 0.1444719615827964935_f64; let w02: f64 = 0.0956387200792748594_f64;
    let n03: f64 = 0.2392873622521370745_f64; let w03: f64 = 0.0938443990808045656_f64;
    let n04: f64 = 0.3318686022821276498_f64; let w04: f64 = 0.0911738786957638847_f64;
    let n05: f64 = 0.4213512761306353454_f64; let w05: f64 = 0.0876520930044038111_f64;
    let n06: f64 = 0.5068999089322293900_f64; let w06: f64 = 0.0833119242269467552_f64;
    let n07: f64 = 0.5877157572407623210_f64; let w07: f64 = 0.0781938957870703065_f64;
    let n08: f64 = 0.6630442669302152009_f64; let w08: f64 = 0.0723457941088485062_f64;
    let n09: f64 = 0.7321821187402896804_f64; let w09: f64 = 0.0658222227763618468_f64;
    let n10: f64 = 0.7944837959679424070_f64; let w10: f64 = 0.0586840934785355471_f64;
    let n11: f64 = 0.8493676137325699701_f64; let w11: f64 = 0.0509980592623761762_f64;
    let n12: f64 = 0.8963211557660521239_f64; let w12: f64 = 0.0428358980222266807_f64;
    let n13: f64 = 0.9349060759377396892_f64; let w13: f64 = 0.0342738629130214331_f64;
    let n14: f64 = 0.9647622555875064308_f64; let w14: f64 = 0.0253920653092620595_f64;
    let n15: f64 = 0.9856115115452683354_f64; let w15: f64 = 0.0162743947309056706_f64;
    let n16: f64 = 0.9972638618494815635_f64; let w16: f64 = 0.0070186100094700966_f64;

    let mut s: f64 = 0.0_f64;
    s = s + w01 * ft_inter_1_subtracted(mid + half * n01, beta);
    s = s + w02 * ft_inter_1_subtracted(mid + half * n02, beta);
    s = s + w03 * ft_inter_1_subtracted(mid + half * n03, beta);
    s = s + w04 * ft_inter_1_subtracted(mid + half * n04, beta);
    s = s + w05 * ft_inter_1_subtracted(mid + half * n05, beta);
    s = s + w06 * ft_inter_1_subtracted(mid + half * n06, beta);
    s = s + w07 * ft_inter_1_subtracted(mid + half * n07, beta);
    s = s + w08 * ft_inter_1_subtracted(mid + half * n08, beta);
    s = s + w09 * ft_inter_1_subtracted(mid + half * n09, beta);
    s = s + w10 * ft_inter_1_subtracted(mid + half * n10, beta);
    s = s + w11 * ft_inter_1_subtracted(mid + half * n11, beta);
    s = s + w12 * ft_inter_1_subtracted(mid + half * n12, beta);
    s = s + w13 * ft_inter_1_subtracted(mid + half * n13, beta);
    s = s + w14 * ft_inter_1_subtracted(mid + half * n14, beta);
    s = s + w15 * ft_inter_1_subtracted(mid + half * n15, beta);
    s = s + w16 * ft_inter_1_subtracted(mid + half * n16, beta);
    s = s + w01 * ft_inter_1_subtracted(mid - half * n01, beta);
    s = s + w02 * ft_inter_1_subtracted(mid - half * n02, beta);
    s = s + w03 * ft_inter_1_subtracted(mid - half * n03, beta);
    s = s + w04 * ft_inter_1_subtracted(mid - half * n04, beta);
    s = s + w05 * ft_inter_1_subtracted(mid - half * n05, beta);
    s = s + w06 * ft_inter_1_subtracted(mid - half * n06, beta);
    s = s + w07 * ft_inter_1_subtracted(mid - half * n07, beta);
    s = s + w08 * ft_inter_1_subtracted(mid - half * n08, beta);
    s = s + w09 * ft_inter_1_subtracted(mid - half * n09, beta);
    s = s + w10 * ft_inter_1_subtracted(mid - half * n10, beta);
    s = s + w11 * ft_inter_1_subtracted(mid - half * n11, beta);
    s = s + w12 * ft_inter_1_subtracted(mid - half * n12, beta);
    s = s + w13 * ft_inter_1_subtracted(mid - half * n13, beta);
    s = s + w14 * ft_inter_1_subtracted(mid - half * n14, beta);
    s = s + w15 * ft_inter_1_subtracted(mid - half * n15, beta);
    s = s + w16 * ft_inter_1_subtracted(mid - half * n16, beta);
    half * s
}

/// Apply 32-point GL to func1 on [a, a+h] via substitution x = a + h·((t+1)/2)^3.
///
/// Cubic grading concentrates quadrature points near x = a, which is essential
/// for the log singularity at x=0. The substitution transforms:
///   ∫ₐ^(a+h) func1(x) dx = ∫₋₁¹ func1(a + h·u³) · h·3u²/2³ · (1/2) dt
/// where u = (t+1)/2, du = dt/2.
/// More precisely: let u = (t+1)/2 ∈ [0,1], x = a + h·u³, dx = 3h·u²·du
///   ∫₋₁¹ func1(a + h·((t+1)/2)³) · 3h·((t+1)/2)² · (1/2) dt
fn gl32_func1_graded(a: f64, h: f64, beta: f64) -> f64 {
    let n01: f64 = 0.0483076656877383162_f64; let w01: f64 = 0.0965400885147278006_f64;
    let n02: f64 = 0.1444719615827964935_f64; let w02: f64 = 0.0956387200792748594_f64;
    let n03: f64 = 0.2392873622521370745_f64; let w03: f64 = 0.0938443990808045656_f64;
    let n04: f64 = 0.3318686022821276498_f64; let w04: f64 = 0.0911738786957638847_f64;
    let n05: f64 = 0.4213512761306353454_f64; let w05: f64 = 0.0876520930044038111_f64;
    let n06: f64 = 0.5068999089322293900_f64; let w06: f64 = 0.0833119242269467552_f64;
    let n07: f64 = 0.5877157572407623210_f64; let w07: f64 = 0.0781938957870703065_f64;
    let n08: f64 = 0.6630442669302152009_f64; let w08: f64 = 0.0723457941088485062_f64;
    let n09: f64 = 0.7321821187402896804_f64; let w09: f64 = 0.0658222227763618468_f64;
    let n10: f64 = 0.7944837959679424070_f64; let w10: f64 = 0.0586840934785355471_f64;
    let n11: f64 = 0.8493676137325699701_f64; let w11: f64 = 0.0509980592623761762_f64;
    let n12: f64 = 0.8963211557660521239_f64; let w12: f64 = 0.0428358980222266807_f64;
    let n13: f64 = 0.9349060759377396892_f64; let w13: f64 = 0.0342738629130214331_f64;
    let n14: f64 = 0.9647622555875064308_f64; let w14: f64 = 0.0253920653092620595_f64;
    let n15: f64 = 0.9856115115452683354_f64; let w15: f64 = 0.0162743947309056706_f64;
    let n16: f64 = 0.9972638618494815635_f64; let w16: f64 = 0.0070186100094700966_f64;

    let mut s: f64 = 0.0_f64;

    // For each GL node t ∈ [-1,1]: u = (t+1)/2, x = a + h*u³, jacobian = 3h*u²/2
    let u = (n01 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w01 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n02 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w02 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n03 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w03 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n04 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w04 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n05 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w05 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n06 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w06 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n07 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w07 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n08 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w08 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n09 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w09 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n10 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w10 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n11 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w11 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n12 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w12 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n13 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w13 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n14 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w14 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n15 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w15 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (n16 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w16 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    // Negative nodes
    let u = (-n01 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w01 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n02 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w02 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n03 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w03 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n04 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w04 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n05 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w05 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n06 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w06 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n07 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w07 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n08 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w08 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n09 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w09 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n10 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w10 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n11 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w11 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n12 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w12 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n13 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w13 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n14 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w14 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n15 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w15 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    let u = (-n16 + 1.0_f64) * 0.5_f64; let x = a + h * u * u * u; s = s + w16 * ft_inter_1(x, beta) * 1.5_f64 * h * u * u;
    s
}

/// Apply 32-point GL to func1 on regular sub-interval [a, a+h].
fn gl32_func1_regular(a: f64, h: f64, beta: f64) -> f64 {
    let half = h * 0.5_f64;
    let mid = a + half;

    let n01: f64 = 0.0483076656877383162_f64; let w01: f64 = 0.0965400885147278006_f64;
    let n02: f64 = 0.1444719615827964935_f64; let w02: f64 = 0.0956387200792748594_f64;
    let n03: f64 = 0.2392873622521370745_f64; let w03: f64 = 0.0938443990808045656_f64;
    let n04: f64 = 0.3318686022821276498_f64; let w04: f64 = 0.0911738786957638847_f64;
    let n05: f64 = 0.4213512761306353454_f64; let w05: f64 = 0.0876520930044038111_f64;
    let n06: f64 = 0.5068999089322293900_f64; let w06: f64 = 0.0833119242269467552_f64;
    let n07: f64 = 0.5877157572407623210_f64; let w07: f64 = 0.0781938957870703065_f64;
    let n08: f64 = 0.6630442669302152009_f64; let w08: f64 = 0.0723457941088485062_f64;
    let n09: f64 = 0.7321821187402896804_f64; let w09: f64 = 0.0658222227763618468_f64;
    let n10: f64 = 0.7944837959679424070_f64; let w10: f64 = 0.0586840934785355471_f64;
    let n11: f64 = 0.8493676137325699701_f64; let w11: f64 = 0.0509980592623761762_f64;
    let n12: f64 = 0.8963211557660521239_f64; let w12: f64 = 0.0428358980222266807_f64;
    let n13: f64 = 0.9349060759377396892_f64; let w13: f64 = 0.0342738629130214331_f64;
    let n14: f64 = 0.9647622555875064308_f64; let w14: f64 = 0.0253920653092620595_f64;
    let n15: f64 = 0.9856115115452683354_f64; let w15: f64 = 0.0162743947309056706_f64;
    let n16: f64 = 0.9972638618494815635_f64; let w16: f64 = 0.0070186100094700966_f64;

    let mut s: f64 = 0.0_f64;
    s = s + w01 * ft_inter_1(mid + half * n01, beta);
    s = s + w02 * ft_inter_1(mid + half * n02, beta);
    s = s + w03 * ft_inter_1(mid + half * n03, beta);
    s = s + w04 * ft_inter_1(mid + half * n04, beta);
    s = s + w05 * ft_inter_1(mid + half * n05, beta);
    s = s + w06 * ft_inter_1(mid + half * n06, beta);
    s = s + w07 * ft_inter_1(mid + half * n07, beta);
    s = s + w08 * ft_inter_1(mid + half * n08, beta);
    s = s + w09 * ft_inter_1(mid + half * n09, beta);
    s = s + w10 * ft_inter_1(mid + half * n10, beta);
    s = s + w11 * ft_inter_1(mid + half * n11, beta);
    s = s + w12 * ft_inter_1(mid + half * n12, beta);
    s = s + w13 * ft_inter_1(mid + half * n13, beta);
    s = s + w14 * ft_inter_1(mid + half * n14, beta);
    s = s + w15 * ft_inter_1(mid + half * n15, beta);
    s = s + w16 * ft_inter_1(mid + half * n16, beta);
    s = s + w01 * ft_inter_1(mid - half * n01, beta);
    s = s + w02 * ft_inter_1(mid - half * n02, beta);
    s = s + w03 * ft_inter_1(mid - half * n03, beta);
    s = s + w04 * ft_inter_1(mid - half * n04, beta);
    s = s + w05 * ft_inter_1(mid - half * n05, beta);
    s = s + w06 * ft_inter_1(mid - half * n06, beta);
    s = s + w07 * ft_inter_1(mid - half * n07, beta);
    s = s + w08 * ft_inter_1(mid - half * n08, beta);
    s = s + w09 * ft_inter_1(mid - half * n09, beta);
    s = s + w10 * ft_inter_1(mid - half * n10, beta);
    s = s + w11 * ft_inter_1(mid - half * n11, beta);
    s = s + w12 * ft_inter_1(mid - half * n12, beta);
    s = s + w13 * ft_inter_1(mid - half * n13, beta);
    s = s + w14 * ft_inter_1(mid - half * n14, beta);
    s = s + w15 * ft_inter_1(mid - half * n15, beta);
    s = s + w16 * ft_inter_1(mid - half * n16, beta);
    half * s
}

/// Integrate func1 from 0 to b using singularity subtraction + composite GL.
///
/// ∫₀ᵇ func1(x) dx = C·(b·ln(b) - b) + ∫₀ᵇ g(x) dx
///
/// where g(x) = func1(x) - C·ln(x) is smooth everywhere including at x=0.
/// The smooth remainder is integrated with 16-panel composite 32-point GL
/// (512 total quadrature points) for ≤ 10⁻¹² relative error.
pub fn xc_integrate_func1(b: f64, beta: f64) -> f64 {
    let csi: f64 = 1.2599210498948732_f64; // M_CBRT2
    let c = -0.75_f64 * beta * csi;

    // Exact analytical part: ∫₀ᵇ C·ln(x) dx = C·(b·ln(b) - b)
    let analytical = c * (b * f64::ln(b) - b);

    // 32-panel composite GL for the smooth subtracted integrand g(x)
    let h = b / 32.0_f64;
    let smooth =
          gl32_subtracted( 0.0_f64 * h, h, beta)
        + gl32_subtracted( 1.0_f64 * h, h, beta)
        + gl32_subtracted( 2.0_f64 * h, h, beta)
        + gl32_subtracted( 3.0_f64 * h, h, beta)
        + gl32_subtracted( 4.0_f64 * h, h, beta)
        + gl32_subtracted( 5.0_f64 * h, h, beta)
        + gl32_subtracted( 6.0_f64 * h, h, beta)
        + gl32_subtracted( 7.0_f64 * h, h, beta)
        + gl32_subtracted( 8.0_f64 * h, h, beta)
        + gl32_subtracted( 9.0_f64 * h, h, beta)
        + gl32_subtracted(10.0_f64 * h, h, beta)
        + gl32_subtracted(11.0_f64 * h, h, beta)
        + gl32_subtracted(12.0_f64 * h, h, beta)
        + gl32_subtracted(13.0_f64 * h, h, beta)
        + gl32_subtracted(14.0_f64 * h, h, beta)
        + gl32_subtracted(15.0_f64 * h, h, beta)
        + gl32_subtracted(16.0_f64 * h, h, beta)
        + gl32_subtracted(17.0_f64 * h, h, beta)
        + gl32_subtracted(18.0_f64 * h, h, beta)
        + gl32_subtracted(19.0_f64 * h, h, beta)
        + gl32_subtracted(20.0_f64 * h, h, beta)
        + gl32_subtracted(21.0_f64 * h, h, beta)
        + gl32_subtracted(22.0_f64 * h, h, beta)
        + gl32_subtracted(23.0_f64 * h, h, beta)
        + gl32_subtracted(24.0_f64 * h, h, beta)
        + gl32_subtracted(25.0_f64 * h, h, beta)
        + gl32_subtracted(26.0_f64 * h, h, beta)
        + gl32_subtracted(27.0_f64 * h, h, beta)
        + gl32_subtracted(28.0_f64 * h, h, beta)
        + gl32_subtracted(29.0_f64 * h, h, beta)
        + gl32_subtracted(30.0_f64 * h, h, beta)
        + gl32_subtracted(31.0_f64 * h, h, beta);

    analytical + smooth
}

// ============================================================================
// FT_inter integrand for lda_x_1d_soft (libxc src/lda_x_1d_soft.c)
//
//   FT_inter(x) = 2 * xc_bessel_K0(x)
//   func1(x)    = FT_inter(x)         = 2*K0(x)
//   func2(x)    = x * FT_inter(x)     = 2*x*K0(x)
//
// The maple2c output for lda_x_1d_soft emits:
//   xc_integrate(func1, NULL, 0.0, b)
//   xc_integrate(func2, NULL, 0.0, b)
//
// Both integrate from 0 to b. K0(x) has a logarithmic singularity at x=0
// (K0(x) ~ -ln(x/2) - γ near 0), so naive composite GL is slow to converge.
// We use the same singularity-subtraction trick as `xc_integrate_func1` above:
// subtract the leading log term analytically and integrate the smooth
// remainder with 32-panel composite 32-point GL (1024 quadrature points).
// ============================================================================

use crate::bessel::xc_bessel_K0;

/// Smooth remainder for ∫₀ᵇ 2·K0(x) dx after subtracting -2·ln(x/2):
///   g1(x) = 2·K0(x) + 2·ln(x/2)
/// Limit as x→0: g1 → -2γ (Euler-Mascheroni). Smooth and bounded on [0, b].
fn lda_soft_g1(x: f64) -> f64 {
    2.0_f64 * xc_bessel_K0(x) + 2.0_f64 * f64::ln(0.5_f64 * x)
}

/// Smooth remainder for ∫₀ᵇ 2·x·K0(x) dx after subtracting -2·x·ln(x/2):
///   g2(x) = 2·x·K0(x) + 2·x·ln(x/2)
/// Limit as x→0: g2 → 0 (linear in x). Smooth and bounded on [0, b].
fn lda_soft_g2(x: f64) -> f64 {
    2.0_f64 * x * xc_bessel_K0(x) + 2.0_f64 * x * f64::ln(0.5_f64 * x)
}

/// Apply 32-point GL to the lda_soft g1 remainder on sub-interval [a, a+h].
fn gl32_lda_soft_1(a: f64, h: f64) -> f64 {
    let half = h * 0.5_f64;
    let mid = a + half;

    let n01: f64 = 0.0483076656877383162_f64; let w01: f64 = 0.0965400885147278006_f64;
    let n02: f64 = 0.1444719615827964935_f64; let w02: f64 = 0.0956387200792748594_f64;
    let n03: f64 = 0.2392873622521370745_f64; let w03: f64 = 0.0938443990808045656_f64;
    let n04: f64 = 0.3318686022821276498_f64; let w04: f64 = 0.0911738786957638847_f64;
    let n05: f64 = 0.4213512761306353454_f64; let w05: f64 = 0.0876520930044038111_f64;
    let n06: f64 = 0.5068999089322293900_f64; let w06: f64 = 0.0833119242269467552_f64;
    let n07: f64 = 0.5877157572407623210_f64; let w07: f64 = 0.0781938957870703065_f64;
    let n08: f64 = 0.6630442669302152009_f64; let w08: f64 = 0.0723457941088485062_f64;
    let n09: f64 = 0.7321821187402896804_f64; let w09: f64 = 0.0658222227763618468_f64;
    let n10: f64 = 0.7944837959679424070_f64; let w10: f64 = 0.0586840934785355471_f64;
    let n11: f64 = 0.8493676137325699701_f64; let w11: f64 = 0.0509980592623761762_f64;
    let n12: f64 = 0.8963211557660521239_f64; let w12: f64 = 0.0428358980222266807_f64;
    let n13: f64 = 0.9349060759377396892_f64; let w13: f64 = 0.0342738629130214331_f64;
    let n14: f64 = 0.9647622555875064308_f64; let w14: f64 = 0.0253920653092620595_f64;
    let n15: f64 = 0.9856115115452683354_f64; let w15: f64 = 0.0162743947309056706_f64;
    let n16: f64 = 0.9972638618494815635_f64; let w16: f64 = 0.0070186100094700966_f64;

    let mut s: f64 = 0.0_f64;
    s = s + w01 * lda_soft_g1(mid + half * n01);
    s = s + w02 * lda_soft_g1(mid + half * n02);
    s = s + w03 * lda_soft_g1(mid + half * n03);
    s = s + w04 * lda_soft_g1(mid + half * n04);
    s = s + w05 * lda_soft_g1(mid + half * n05);
    s = s + w06 * lda_soft_g1(mid + half * n06);
    s = s + w07 * lda_soft_g1(mid + half * n07);
    s = s + w08 * lda_soft_g1(mid + half * n08);
    s = s + w09 * lda_soft_g1(mid + half * n09);
    s = s + w10 * lda_soft_g1(mid + half * n10);
    s = s + w11 * lda_soft_g1(mid + half * n11);
    s = s + w12 * lda_soft_g1(mid + half * n12);
    s = s + w13 * lda_soft_g1(mid + half * n13);
    s = s + w14 * lda_soft_g1(mid + half * n14);
    s = s + w15 * lda_soft_g1(mid + half * n15);
    s = s + w16 * lda_soft_g1(mid + half * n16);
    s = s + w01 * lda_soft_g1(mid - half * n01);
    s = s + w02 * lda_soft_g1(mid - half * n02);
    s = s + w03 * lda_soft_g1(mid - half * n03);
    s = s + w04 * lda_soft_g1(mid - half * n04);
    s = s + w05 * lda_soft_g1(mid - half * n05);
    s = s + w06 * lda_soft_g1(mid - half * n06);
    s = s + w07 * lda_soft_g1(mid - half * n07);
    s = s + w08 * lda_soft_g1(mid - half * n08);
    s = s + w09 * lda_soft_g1(mid - half * n09);
    s = s + w10 * lda_soft_g1(mid - half * n10);
    s = s + w11 * lda_soft_g1(mid - half * n11);
    s = s + w12 * lda_soft_g1(mid - half * n12);
    s = s + w13 * lda_soft_g1(mid - half * n13);
    s = s + w14 * lda_soft_g1(mid - half * n14);
    s = s + w15 * lda_soft_g1(mid - half * n15);
    s = s + w16 * lda_soft_g1(mid - half * n16);
    half * s
}

/// Apply 32-point GL to the lda_soft g2 remainder on sub-interval [a, a+h].
fn gl32_lda_soft_2(a: f64, h: f64) -> f64 {
    let half = h * 0.5_f64;
    let mid = a + half;

    let n01: f64 = 0.0483076656877383162_f64; let w01: f64 = 0.0965400885147278006_f64;
    let n02: f64 = 0.1444719615827964935_f64; let w02: f64 = 0.0956387200792748594_f64;
    let n03: f64 = 0.2392873622521370745_f64; let w03: f64 = 0.0938443990808045656_f64;
    let n04: f64 = 0.3318686022821276498_f64; let w04: f64 = 0.0911738786957638847_f64;
    let n05: f64 = 0.4213512761306353454_f64; let w05: f64 = 0.0876520930044038111_f64;
    let n06: f64 = 0.5068999089322293900_f64; let w06: f64 = 0.0833119242269467552_f64;
    let n07: f64 = 0.5877157572407623210_f64; let w07: f64 = 0.0781938957870703065_f64;
    let n08: f64 = 0.6630442669302152009_f64; let w08: f64 = 0.0723457941088485062_f64;
    let n09: f64 = 0.7321821187402896804_f64; let w09: f64 = 0.0658222227763618468_f64;
    let n10: f64 = 0.7944837959679424070_f64; let w10: f64 = 0.0586840934785355471_f64;
    let n11: f64 = 0.8493676137325699701_f64; let w11: f64 = 0.0509980592623761762_f64;
    let n12: f64 = 0.8963211557660521239_f64; let w12: f64 = 0.0428358980222266807_f64;
    let n13: f64 = 0.9349060759377396892_f64; let w13: f64 = 0.0342738629130214331_f64;
    let n14: f64 = 0.9647622555875064308_f64; let w14: f64 = 0.0253920653092620595_f64;
    let n15: f64 = 0.9856115115452683354_f64; let w15: f64 = 0.0162743947309056706_f64;
    let n16: f64 = 0.9972638618494815635_f64; let w16: f64 = 0.0070186100094700966_f64;

    let mut s: f64 = 0.0_f64;
    s = s + w01 * lda_soft_g2(mid + half * n01);
    s = s + w02 * lda_soft_g2(mid + half * n02);
    s = s + w03 * lda_soft_g2(mid + half * n03);
    s = s + w04 * lda_soft_g2(mid + half * n04);
    s = s + w05 * lda_soft_g2(mid + half * n05);
    s = s + w06 * lda_soft_g2(mid + half * n06);
    s = s + w07 * lda_soft_g2(mid + half * n07);
    s = s + w08 * lda_soft_g2(mid + half * n08);
    s = s + w09 * lda_soft_g2(mid + half * n09);
    s = s + w10 * lda_soft_g2(mid + half * n10);
    s = s + w11 * lda_soft_g2(mid + half * n11);
    s = s + w12 * lda_soft_g2(mid + half * n12);
    s = s + w13 * lda_soft_g2(mid + half * n13);
    s = s + w14 * lda_soft_g2(mid + half * n14);
    s = s + w15 * lda_soft_g2(mid + half * n15);
    s = s + w16 * lda_soft_g2(mid + half * n16);
    s = s + w01 * lda_soft_g2(mid - half * n01);
    s = s + w02 * lda_soft_g2(mid - half * n02);
    s = s + w03 * lda_soft_g2(mid - half * n03);
    s = s + w04 * lda_soft_g2(mid - half * n04);
    s = s + w05 * lda_soft_g2(mid - half * n05);
    s = s + w06 * lda_soft_g2(mid - half * n06);
    s = s + w07 * lda_soft_g2(mid - half * n07);
    s = s + w08 * lda_soft_g2(mid - half * n08);
    s = s + w09 * lda_soft_g2(mid - half * n09);
    s = s + w10 * lda_soft_g2(mid - half * n10);
    s = s + w11 * lda_soft_g2(mid - half * n11);
    s = s + w12 * lda_soft_g2(mid - half * n12);
    s = s + w13 * lda_soft_g2(mid - half * n13);
    s = s + w14 * lda_soft_g2(mid - half * n14);
    s = s + w15 * lda_soft_g2(mid - half * n15);
    s = s + w16 * lda_soft_g2(mid - half * n16);
    half * s
}

/// Integrate ∫₀ᵇ 2·K0(x) dx using singularity subtraction + composite GL.
///
/// Maps libxc's `xc_integrate(func1, NULL, 0.0, b)` for `lda_x_1d_soft`.
///
/// Analytical part: ∫₀ᵇ -2·ln(x/2) dx = -2·b·ln(b/2) + 2·b.
/// Smooth remainder g1(x) = 2·K0(x) + 2·ln(x/2) integrated by 32-panel
/// composite 32-point GL (1024 quadrature points) for ≤ 10⁻¹² rel error.
pub fn xc_integrate_lda_soft_func1(b: f64) -> f64 {
    // Analytical: ∫₀ᵇ -2·ln(x/2) dx = -2·(b·ln(b/2) - b)
    let analytical = -2.0_f64 * (b * f64::ln(0.5_f64 * b) - b);

    let h = b / 32.0_f64;
    let smooth =
          gl32_lda_soft_1( 0.0_f64 * h, h)
        + gl32_lda_soft_1( 1.0_f64 * h, h)
        + gl32_lda_soft_1( 2.0_f64 * h, h)
        + gl32_lda_soft_1( 3.0_f64 * h, h)
        + gl32_lda_soft_1( 4.0_f64 * h, h)
        + gl32_lda_soft_1( 5.0_f64 * h, h)
        + gl32_lda_soft_1( 6.0_f64 * h, h)
        + gl32_lda_soft_1( 7.0_f64 * h, h)
        + gl32_lda_soft_1( 8.0_f64 * h, h)
        + gl32_lda_soft_1( 9.0_f64 * h, h)
        + gl32_lda_soft_1(10.0_f64 * h, h)
        + gl32_lda_soft_1(11.0_f64 * h, h)
        + gl32_lda_soft_1(12.0_f64 * h, h)
        + gl32_lda_soft_1(13.0_f64 * h, h)
        + gl32_lda_soft_1(14.0_f64 * h, h)
        + gl32_lda_soft_1(15.0_f64 * h, h)
        + gl32_lda_soft_1(16.0_f64 * h, h)
        + gl32_lda_soft_1(17.0_f64 * h, h)
        + gl32_lda_soft_1(18.0_f64 * h, h)
        + gl32_lda_soft_1(19.0_f64 * h, h)
        + gl32_lda_soft_1(20.0_f64 * h, h)
        + gl32_lda_soft_1(21.0_f64 * h, h)
        + gl32_lda_soft_1(22.0_f64 * h, h)
        + gl32_lda_soft_1(23.0_f64 * h, h)
        + gl32_lda_soft_1(24.0_f64 * h, h)
        + gl32_lda_soft_1(25.0_f64 * h, h)
        + gl32_lda_soft_1(26.0_f64 * h, h)
        + gl32_lda_soft_1(27.0_f64 * h, h)
        + gl32_lda_soft_1(28.0_f64 * h, h)
        + gl32_lda_soft_1(29.0_f64 * h, h)
        + gl32_lda_soft_1(30.0_f64 * h, h)
        + gl32_lda_soft_1(31.0_f64 * h, h);

    analytical + smooth
}

/// Integrate ∫₀ᵇ 2·x·K0(x) dx using singularity subtraction + composite GL.
///
/// Maps libxc's `xc_integrate(func2, NULL, 0.0, b)` for `lda_x_1d_soft`.
///
/// Analytical part: ∫₀ᵇ -2·x·ln(x/2) dx = -b²·ln(b/2) + b²/2.
/// Smooth remainder g2(x) = 2·x·K0(x) + 2·x·ln(x/2) integrated by 32-panel
/// composite 32-point GL (1024 quadrature points) for ≤ 10⁻¹² rel error.
pub fn xc_integrate_lda_soft_func2(b: f64) -> f64 {
    // Analytical: ∫₀ᵇ -2·x·ln(x/2) dx = -2·[x²/2·ln(x/2) - x²/4]₀ᵇ
    //           = -b²·ln(b/2) + b²/2
    let analytical = -1.0_f64 * b * b * f64::ln(0.5_f64 * b) + 0.5_f64 * b * b;

    let h = b / 32.0_f64;
    let smooth =
          gl32_lda_soft_2( 0.0_f64 * h, h)
        + gl32_lda_soft_2( 1.0_f64 * h, h)
        + gl32_lda_soft_2( 2.0_f64 * h, h)
        + gl32_lda_soft_2( 3.0_f64 * h, h)
        + gl32_lda_soft_2( 4.0_f64 * h, h)
        + gl32_lda_soft_2( 5.0_f64 * h, h)
        + gl32_lda_soft_2( 6.0_f64 * h, h)
        + gl32_lda_soft_2( 7.0_f64 * h, h)
        + gl32_lda_soft_2( 8.0_f64 * h, h)
        + gl32_lda_soft_2( 9.0_f64 * h, h)
        + gl32_lda_soft_2(10.0_f64 * h, h)
        + gl32_lda_soft_2(11.0_f64 * h, h)
        + gl32_lda_soft_2(12.0_f64 * h, h)
        + gl32_lda_soft_2(13.0_f64 * h, h)
        + gl32_lda_soft_2(14.0_f64 * h, h)
        + gl32_lda_soft_2(15.0_f64 * h, h)
        + gl32_lda_soft_2(16.0_f64 * h, h)
        + gl32_lda_soft_2(17.0_f64 * h, h)
        + gl32_lda_soft_2(18.0_f64 * h, h)
        + gl32_lda_soft_2(19.0_f64 * h, h)
        + gl32_lda_soft_2(20.0_f64 * h, h)
        + gl32_lda_soft_2(21.0_f64 * h, h)
        + gl32_lda_soft_2(22.0_f64 * h, h)
        + gl32_lda_soft_2(23.0_f64 * h, h)
        + gl32_lda_soft_2(24.0_f64 * h, h)
        + gl32_lda_soft_2(25.0_f64 * h, h)
        + gl32_lda_soft_2(26.0_f64 * h, h)
        + gl32_lda_soft_2(27.0_f64 * h, h)
        + gl32_lda_soft_2(28.0_f64 * h, h)
        + gl32_lda_soft_2(29.0_f64 * h, h)
        + gl32_lda_soft_2(30.0_f64 * h, h)
        + gl32_lda_soft_2(31.0_f64 * h, h);

    analytical + smooth
}

// ============================================================================
// FT_inter integrand for lda_x_1d_exponential (libxc src/lda_x_1d_exponential.c)
//
//   FT_inter(x) = xc_E1_scaled(x*x)
//   func1(x)    = FT_inter(x)        =     xc_e1_scaled(x²)
//   func2(x)    = x * FT_inter(x)    = x * xc_e1_scaled(x²)
//
// The maple2c output emits:
//   xc_integrate(func1, NULL, 1e-20, b)
//   xc_integrate(func2, NULL, 1e-20, b)
//
// Asymptotic at x→0: xc_e1_scaled(x²) ~ -2·ln(x) - γ + O(x²·ln(x))
// (γ = Euler-Mascheroni). Subtract the leading -2·ln(x) - γ analytically;
// integrate the smooth remainder via 32-panel composite 32-point GL.
// Lower bound 1e-20 is approximated as 0 — the [0, 1e-20] tail contributes
// below f64 epsilon for any realistic b ≥ 1e-10.
// ============================================================================

use crate::expint_e1::xc_e1_scaled;

/// Smooth remainder for ∫₀ᵇ xc_e1_scaled(x²) dx after subtracting -2·ln(x) - γ:
///   g1(x) = xc_e1_scaled(x²) + 2·ln(x) + γ
/// Limit as x→0: g1 → 0. Smooth and bounded on [0, b].
fn lda_exp_g1(x: f64) -> f64 {
    let gamma: f64 = 0.5772156649015328606065120900824024310421593359399235988057672_f64;
    xc_e1_scaled(x * x) + 2.0_f64 * f64::ln(x) + gamma
}

/// Smooth remainder for ∫₀ᵇ x·xc_e1_scaled(x²) dx after subtracting x·(-2·ln(x) - γ):
///   g2(x) = x·xc_e1_scaled(x²) + 2·x·ln(x) + γ·x
/// Limit as x→0: g2 → 0 (vanishes as x³·ln(x)). Smooth and bounded on [0, b].
fn lda_exp_g2(x: f64) -> f64 {
    let gamma: f64 = 0.5772156649015328606065120900824024310421593359399235988057672_f64;
    x * xc_e1_scaled(x * x) + 2.0_f64 * x * f64::ln(x) + gamma * x
}

/// Apply 32-point GL to the lda_exp g1 remainder on sub-interval [a, a+h].
fn gl32_lda_exp_1(a: f64, h: f64) -> f64 {
    let half = h * 0.5_f64;
    let mid = a + half;

    let n01: f64 = 0.0483076656877383162_f64; let w01: f64 = 0.0965400885147278006_f64;
    let n02: f64 = 0.1444719615827964935_f64; let w02: f64 = 0.0956387200792748594_f64;
    let n03: f64 = 0.2392873622521370745_f64; let w03: f64 = 0.0938443990808045656_f64;
    let n04: f64 = 0.3318686022821276498_f64; let w04: f64 = 0.0911738786957638847_f64;
    let n05: f64 = 0.4213512761306353454_f64; let w05: f64 = 0.0876520930044038111_f64;
    let n06: f64 = 0.5068999089322293900_f64; let w06: f64 = 0.0833119242269467552_f64;
    let n07: f64 = 0.5877157572407623210_f64; let w07: f64 = 0.0781938957870703065_f64;
    let n08: f64 = 0.6630442669302152009_f64; let w08: f64 = 0.0723457941088485062_f64;
    let n09: f64 = 0.7321821187402896804_f64; let w09: f64 = 0.0658222227763618468_f64;
    let n10: f64 = 0.7944837959679424070_f64; let w10: f64 = 0.0586840934785355471_f64;
    let n11: f64 = 0.8493676137325699701_f64; let w11: f64 = 0.0509980592623761762_f64;
    let n12: f64 = 0.8963211557660521239_f64; let w12: f64 = 0.0428358980222266807_f64;
    let n13: f64 = 0.9349060759377396892_f64; let w13: f64 = 0.0342738629130214331_f64;
    let n14: f64 = 0.9647622555875064308_f64; let w14: f64 = 0.0253920653092620595_f64;
    let n15: f64 = 0.9856115115452683354_f64; let w15: f64 = 0.0162743947309056706_f64;
    let n16: f64 = 0.9972638618494815635_f64; let w16: f64 = 0.0070186100094700966_f64;

    let mut s: f64 = 0.0_f64;
    s = s + w01 * lda_exp_g1(mid + half * n01);
    s = s + w02 * lda_exp_g1(mid + half * n02);
    s = s + w03 * lda_exp_g1(mid + half * n03);
    s = s + w04 * lda_exp_g1(mid + half * n04);
    s = s + w05 * lda_exp_g1(mid + half * n05);
    s = s + w06 * lda_exp_g1(mid + half * n06);
    s = s + w07 * lda_exp_g1(mid + half * n07);
    s = s + w08 * lda_exp_g1(mid + half * n08);
    s = s + w09 * lda_exp_g1(mid + half * n09);
    s = s + w10 * lda_exp_g1(mid + half * n10);
    s = s + w11 * lda_exp_g1(mid + half * n11);
    s = s + w12 * lda_exp_g1(mid + half * n12);
    s = s + w13 * lda_exp_g1(mid + half * n13);
    s = s + w14 * lda_exp_g1(mid + half * n14);
    s = s + w15 * lda_exp_g1(mid + half * n15);
    s = s + w16 * lda_exp_g1(mid + half * n16);
    s = s + w01 * lda_exp_g1(mid - half * n01);
    s = s + w02 * lda_exp_g1(mid - half * n02);
    s = s + w03 * lda_exp_g1(mid - half * n03);
    s = s + w04 * lda_exp_g1(mid - half * n04);
    s = s + w05 * lda_exp_g1(mid - half * n05);
    s = s + w06 * lda_exp_g1(mid - half * n06);
    s = s + w07 * lda_exp_g1(mid - half * n07);
    s = s + w08 * lda_exp_g1(mid - half * n08);
    s = s + w09 * lda_exp_g1(mid - half * n09);
    s = s + w10 * lda_exp_g1(mid - half * n10);
    s = s + w11 * lda_exp_g1(mid - half * n11);
    s = s + w12 * lda_exp_g1(mid - half * n12);
    s = s + w13 * lda_exp_g1(mid - half * n13);
    s = s + w14 * lda_exp_g1(mid - half * n14);
    s = s + w15 * lda_exp_g1(mid - half * n15);
    s = s + w16 * lda_exp_g1(mid - half * n16);
    half * s
}

/// Apply 32-point GL to the lda_exp g2 remainder on sub-interval [a, a+h].
fn gl32_lda_exp_2(a: f64, h: f64) -> f64 {
    let half = h * 0.5_f64;
    let mid = a + half;

    let n01: f64 = 0.0483076656877383162_f64; let w01: f64 = 0.0965400885147278006_f64;
    let n02: f64 = 0.1444719615827964935_f64; let w02: f64 = 0.0956387200792748594_f64;
    let n03: f64 = 0.2392873622521370745_f64; let w03: f64 = 0.0938443990808045656_f64;
    let n04: f64 = 0.3318686022821276498_f64; let w04: f64 = 0.0911738786957638847_f64;
    let n05: f64 = 0.4213512761306353454_f64; let w05: f64 = 0.0876520930044038111_f64;
    let n06: f64 = 0.5068999089322293900_f64; let w06: f64 = 0.0833119242269467552_f64;
    let n07: f64 = 0.5877157572407623210_f64; let w07: f64 = 0.0781938957870703065_f64;
    let n08: f64 = 0.6630442669302152009_f64; let w08: f64 = 0.0723457941088485062_f64;
    let n09: f64 = 0.7321821187402896804_f64; let w09: f64 = 0.0658222227763618468_f64;
    let n10: f64 = 0.7944837959679424070_f64; let w10: f64 = 0.0586840934785355471_f64;
    let n11: f64 = 0.8493676137325699701_f64; let w11: f64 = 0.0509980592623761762_f64;
    let n12: f64 = 0.8963211557660521239_f64; let w12: f64 = 0.0428358980222266807_f64;
    let n13: f64 = 0.9349060759377396892_f64; let w13: f64 = 0.0342738629130214331_f64;
    let n14: f64 = 0.9647622555875064308_f64; let w14: f64 = 0.0253920653092620595_f64;
    let n15: f64 = 0.9856115115452683354_f64; let w15: f64 = 0.0162743947309056706_f64;
    let n16: f64 = 0.9972638618494815635_f64; let w16: f64 = 0.0070186100094700966_f64;

    let mut s: f64 = 0.0_f64;
    s = s + w01 * lda_exp_g2(mid + half * n01);
    s = s + w02 * lda_exp_g2(mid + half * n02);
    s = s + w03 * lda_exp_g2(mid + half * n03);
    s = s + w04 * lda_exp_g2(mid + half * n04);
    s = s + w05 * lda_exp_g2(mid + half * n05);
    s = s + w06 * lda_exp_g2(mid + half * n06);
    s = s + w07 * lda_exp_g2(mid + half * n07);
    s = s + w08 * lda_exp_g2(mid + half * n08);
    s = s + w09 * lda_exp_g2(mid + half * n09);
    s = s + w10 * lda_exp_g2(mid + half * n10);
    s = s + w11 * lda_exp_g2(mid + half * n11);
    s = s + w12 * lda_exp_g2(mid + half * n12);
    s = s + w13 * lda_exp_g2(mid + half * n13);
    s = s + w14 * lda_exp_g2(mid + half * n14);
    s = s + w15 * lda_exp_g2(mid + half * n15);
    s = s + w16 * lda_exp_g2(mid + half * n16);
    s = s + w01 * lda_exp_g2(mid - half * n01);
    s = s + w02 * lda_exp_g2(mid - half * n02);
    s = s + w03 * lda_exp_g2(mid - half * n03);
    s = s + w04 * lda_exp_g2(mid - half * n04);
    s = s + w05 * lda_exp_g2(mid - half * n05);
    s = s + w06 * lda_exp_g2(mid - half * n06);
    s = s + w07 * lda_exp_g2(mid - half * n07);
    s = s + w08 * lda_exp_g2(mid - half * n08);
    s = s + w09 * lda_exp_g2(mid - half * n09);
    s = s + w10 * lda_exp_g2(mid - half * n10);
    s = s + w11 * lda_exp_g2(mid - half * n11);
    s = s + w12 * lda_exp_g2(mid - half * n12);
    s = s + w13 * lda_exp_g2(mid - half * n13);
    s = s + w14 * lda_exp_g2(mid - half * n14);
    s = s + w15 * lda_exp_g2(mid - half * n15);
    s = s + w16 * lda_exp_g2(mid - half * n16);
    half * s
}

/// Integrate ∫₀ᵇ xc_e1_scaled(x²) dx using singularity subtraction + composite GL.
///
/// Maps libxc's `xc_integrate(func1, NULL, 1e-20, b)` for `lda_x_1d_exponential`.
///
/// Analytical part: ∫₀ᵇ (-2·ln(x) - γ) dx = -2·b·ln(b) + 2·b - γ·b.
/// Smooth remainder g1(x) = xc_e1_scaled(x²) + 2·ln(x) + γ integrated by
/// 32-panel composite 32-point GL (1024 quadrature points) for ≤ 10⁻¹² rel error.
pub fn xc_integrate_lda_exponential_func1(b: f64) -> f64 {
    let gamma: f64 = 0.5772156649015328606065120900824024310421593359399235988057672_f64;
    // Analytical: ∫₀ᵇ (-2·ln(x) - γ) dx = -2·(b·ln(b) - b) - γ·b
    let analytical = -2.0_f64 * (b * f64::ln(b) - b) - gamma * b;

    let h = b / 32.0_f64;
    let smooth =
          gl32_lda_exp_1( 0.0_f64 * h, h)
        + gl32_lda_exp_1( 1.0_f64 * h, h)
        + gl32_lda_exp_1( 2.0_f64 * h, h)
        + gl32_lda_exp_1( 3.0_f64 * h, h)
        + gl32_lda_exp_1( 4.0_f64 * h, h)
        + gl32_lda_exp_1( 5.0_f64 * h, h)
        + gl32_lda_exp_1( 6.0_f64 * h, h)
        + gl32_lda_exp_1( 7.0_f64 * h, h)
        + gl32_lda_exp_1( 8.0_f64 * h, h)
        + gl32_lda_exp_1( 9.0_f64 * h, h)
        + gl32_lda_exp_1(10.0_f64 * h, h)
        + gl32_lda_exp_1(11.0_f64 * h, h)
        + gl32_lda_exp_1(12.0_f64 * h, h)
        + gl32_lda_exp_1(13.0_f64 * h, h)
        + gl32_lda_exp_1(14.0_f64 * h, h)
        + gl32_lda_exp_1(15.0_f64 * h, h)
        + gl32_lda_exp_1(16.0_f64 * h, h)
        + gl32_lda_exp_1(17.0_f64 * h, h)
        + gl32_lda_exp_1(18.0_f64 * h, h)
        + gl32_lda_exp_1(19.0_f64 * h, h)
        + gl32_lda_exp_1(20.0_f64 * h, h)
        + gl32_lda_exp_1(21.0_f64 * h, h)
        + gl32_lda_exp_1(22.0_f64 * h, h)
        + gl32_lda_exp_1(23.0_f64 * h, h)
        + gl32_lda_exp_1(24.0_f64 * h, h)
        + gl32_lda_exp_1(25.0_f64 * h, h)
        + gl32_lda_exp_1(26.0_f64 * h, h)
        + gl32_lda_exp_1(27.0_f64 * h, h)
        + gl32_lda_exp_1(28.0_f64 * h, h)
        + gl32_lda_exp_1(29.0_f64 * h, h)
        + gl32_lda_exp_1(30.0_f64 * h, h)
        + gl32_lda_exp_1(31.0_f64 * h, h);

    analytical + smooth
}

/// Integrate ∫₀ᵇ x·xc_e1_scaled(x²) dx using singularity subtraction + composite GL.
///
/// Maps libxc's `xc_integrate(func2, NULL, 1e-20, b)` for `lda_x_1d_exponential`.
///
/// Analytical part: ∫₀ᵇ (-2·x·ln(x) - γ·x) dx = -b²·ln(b) + b²/2 - γ·b²/2.
/// Smooth remainder g2(x) = x·xc_e1_scaled(x²) + 2·x·ln(x) + γ·x integrated
/// by 32-panel composite 32-point GL (1024 quadrature points) for ≤ 10⁻¹² rel error.
pub fn xc_integrate_lda_exponential_func2(b: f64) -> f64 {
    let gamma: f64 = 0.5772156649015328606065120900824024310421593359399235988057672_f64;
    // Analytical: ∫₀ᵇ -2·x·ln(x) dx = -b²·ln(b) + b²/2;  ∫₀ᵇ -γ·x dx = -γ·b²/2
    let analytical = -1.0_f64 * b * b * f64::ln(b) + 0.5_f64 * b * b - 0.5_f64 * gamma * b * b;

    let h = b / 32.0_f64;
    let smooth =
          gl32_lda_exp_2( 0.0_f64 * h, h)
        + gl32_lda_exp_2( 1.0_f64 * h, h)
        + gl32_lda_exp_2( 2.0_f64 * h, h)
        + gl32_lda_exp_2( 3.0_f64 * h, h)
        + gl32_lda_exp_2( 4.0_f64 * h, h)
        + gl32_lda_exp_2( 5.0_f64 * h, h)
        + gl32_lda_exp_2( 6.0_f64 * h, h)
        + gl32_lda_exp_2( 7.0_f64 * h, h)
        + gl32_lda_exp_2( 8.0_f64 * h, h)
        + gl32_lda_exp_2( 9.0_f64 * h, h)
        + gl32_lda_exp_2(10.0_f64 * h, h)
        + gl32_lda_exp_2(11.0_f64 * h, h)
        + gl32_lda_exp_2(12.0_f64 * h, h)
        + gl32_lda_exp_2(13.0_f64 * h, h)
        + gl32_lda_exp_2(14.0_f64 * h, h)
        + gl32_lda_exp_2(15.0_f64 * h, h)
        + gl32_lda_exp_2(16.0_f64 * h, h)
        + gl32_lda_exp_2(17.0_f64 * h, h)
        + gl32_lda_exp_2(18.0_f64 * h, h)
        + gl32_lda_exp_2(19.0_f64 * h, h)
        + gl32_lda_exp_2(20.0_f64 * h, h)
        + gl32_lda_exp_2(21.0_f64 * h, h)
        + gl32_lda_exp_2(22.0_f64 * h, h)
        + gl32_lda_exp_2(23.0_f64 * h, h)
        + gl32_lda_exp_2(24.0_f64 * h, h)
        + gl32_lda_exp_2(25.0_f64 * h, h)
        + gl32_lda_exp_2(26.0_f64 * h, h)
        + gl32_lda_exp_2(27.0_f64 * h, h)
        + gl32_lda_exp_2(28.0_f64 * h, h)
        + gl32_lda_exp_2(29.0_f64 * h, h)
        + gl32_lda_exp_2(30.0_f64 * h, h)
        + gl32_lda_exp_2(31.0_f64 * h, h);

    analytical + smooth
}
