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

use cubecl::prelude::*;

// ============================================================================
// FT_inter integrand (from libxc gga_x_fd_lb94.c)
// ============================================================================

/// The FT_inter integrand for n=0 (func0):
///   f(x) = -3/4 * beta * csi / (1 + 3*beta*csi*x*ln(csi*x + sqrt(csi²x² + 1)))
/// where csi = 2^(1/3) = M_CBRT2.
#[cube]
fn ft_inter_0<F: Float>(x: F, beta: F) -> F {
    let csi: F = F::cast_from(1.2599210498948732_f64); // M_CBRT2
    let num = F::new(-0.75) * beta * csi;
    let csi_x = csi * x;
    let denom = F::new(1.0) + F::new(3.0) * beta * csi * x * F::ln(csi_x + F::sqrt(csi_x * csi_x + F::new(1.0)));
    num / denom
}

/// The FT_inter integrand for n=1 (func1):
///   f(x) = -3/4 * beta * csi * ln(x) / (1 + 3*beta*csi*x*ln(csi*x + sqrt(csi²x² + 1)))
/// Has a logarithmic singularity at x=0 (integrable).
#[cube]
fn ft_inter_1<F: Float>(x: F, beta: F) -> F {
    let csi: F = F::cast_from(1.2599210498948732_f64); // M_CBRT2
    let num = F::new(-0.75) * beta * csi * F::ln(x);
    let csi_x = csi * x;
    let denom = F::new(1.0) + F::new(3.0) * beta * csi * x * F::ln(csi_x + F::sqrt(csi_x * csi_x + F::new(1.0)));
    num / denom
}

// ============================================================================
// 32-point Gauss-Legendre nodes and weights on [-1, 1]
// ============================================================================

// To integrate f on [a, b]: ∫f(x)dx ≈ (b-a)/2 * Σ wᵢ f((b-a)/2 * xᵢ + (a+b)/2)
// We inline the 32 nodes/weights as constants since CubeCL can't use arrays.

/// Apply 32-point GL to func0 on sub-interval [a, a+h].
#[cube]
fn gl32_func0<F: Float>(a: F, h: F, beta: F) -> F {
    let half = h * F::new(0.5);
    let mid = a + half;

    let n01: F = F::cast_from(0.0483076656877383162_f64); let w01: F = F::cast_from(0.0965400885147278006_f64);
    let n02: F = F::cast_from(0.1444719615827964935_f64); let w02: F = F::cast_from(0.0956387200792748594_f64);
    let n03: F = F::cast_from(0.2392873622521370745_f64); let w03: F = F::cast_from(0.0938443990808045656_f64);
    let n04: F = F::cast_from(0.3318686022821276498_f64); let w04: F = F::cast_from(0.0911738786957638847_f64);
    let n05: F = F::cast_from(0.4213512761306353454_f64); let w05: F = F::cast_from(0.0876520930044038111_f64);
    let n06: F = F::cast_from(0.5068999089322293900_f64); let w06: F = F::cast_from(0.0833119242269467552_f64);
    let n07: F = F::cast_from(0.5877157572407623210_f64); let w07: F = F::cast_from(0.0781938957870703065_f64);
    let n08: F = F::cast_from(0.6630442669302152009_f64); let w08: F = F::cast_from(0.0723457941088485062_f64);
    let n09: F = F::cast_from(0.7321821187402896804_f64); let w09: F = F::cast_from(0.0658222227763618468_f64);
    let n10: F = F::cast_from(0.7944837959679424070_f64); let w10: F = F::cast_from(0.0586840934785355471_f64);
    let n11: F = F::cast_from(0.8493676137325699701_f64); let w11: F = F::cast_from(0.0509980592623761762_f64);
    let n12: F = F::cast_from(0.8963211557660521239_f64); let w12: F = F::cast_from(0.0428358980222266807_f64);
    let n13: F = F::cast_from(0.9349060759377396892_f64); let w13: F = F::cast_from(0.0342738629130214331_f64);
    let n14: F = F::cast_from(0.9647622555875064308_f64); let w14: F = F::cast_from(0.0253920653092620595_f64);
    let n15: F = F::cast_from(0.9856115115452683354_f64); let w15: F = F::cast_from(0.0162743947309056706_f64);
    let n16: F = F::cast_from(0.9972638618494815635_f64); let w16: F = F::cast_from(0.0070186100094700966_f64);

    let mut s: F = F::new(0.0);
    s = s + w01 * ft_inter_0::<F>(mid + half * n01, beta);
    s = s + w02 * ft_inter_0::<F>(mid + half * n02, beta);
    s = s + w03 * ft_inter_0::<F>(mid + half * n03, beta);
    s = s + w04 * ft_inter_0::<F>(mid + half * n04, beta);
    s = s + w05 * ft_inter_0::<F>(mid + half * n05, beta);
    s = s + w06 * ft_inter_0::<F>(mid + half * n06, beta);
    s = s + w07 * ft_inter_0::<F>(mid + half * n07, beta);
    s = s + w08 * ft_inter_0::<F>(mid + half * n08, beta);
    s = s + w09 * ft_inter_0::<F>(mid + half * n09, beta);
    s = s + w10 * ft_inter_0::<F>(mid + half * n10, beta);
    s = s + w11 * ft_inter_0::<F>(mid + half * n11, beta);
    s = s + w12 * ft_inter_0::<F>(mid + half * n12, beta);
    s = s + w13 * ft_inter_0::<F>(mid + half * n13, beta);
    s = s + w14 * ft_inter_0::<F>(mid + half * n14, beta);
    s = s + w15 * ft_inter_0::<F>(mid + half * n15, beta);
    s = s + w16 * ft_inter_0::<F>(mid + half * n16, beta);
    s = s + w01 * ft_inter_0::<F>(mid - half * n01, beta);
    s = s + w02 * ft_inter_0::<F>(mid - half * n02, beta);
    s = s + w03 * ft_inter_0::<F>(mid - half * n03, beta);
    s = s + w04 * ft_inter_0::<F>(mid - half * n04, beta);
    s = s + w05 * ft_inter_0::<F>(mid - half * n05, beta);
    s = s + w06 * ft_inter_0::<F>(mid - half * n06, beta);
    s = s + w07 * ft_inter_0::<F>(mid - half * n07, beta);
    s = s + w08 * ft_inter_0::<F>(mid - half * n08, beta);
    s = s + w09 * ft_inter_0::<F>(mid - half * n09, beta);
    s = s + w10 * ft_inter_0::<F>(mid - half * n10, beta);
    s = s + w11 * ft_inter_0::<F>(mid - half * n11, beta);
    s = s + w12 * ft_inter_0::<F>(mid - half * n12, beta);
    s = s + w13 * ft_inter_0::<F>(mid - half * n13, beta);
    s = s + w14 * ft_inter_0::<F>(mid - half * n14, beta);
    s = s + w15 * ft_inter_0::<F>(mid - half * n15, beta);
    s = s + w16 * ft_inter_0::<F>(mid - half * n16, beta);
    half * s
}

/// Integrate func0 from 0 to b using 16-panel composite 32-point GL (512 points).
#[cube]
pub fn xc_integrate_func0<F: Float>(b: F, beta: F) -> F {
    let h = b / F::new(16.0);
    gl32_func0::<F>( F::new( 0.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 1.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 2.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 3.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 4.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 5.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 6.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 7.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 8.0) * h, h, beta)
        + gl32_func0::<F>( F::new( 9.0) * h, h, beta)
        + gl32_func0::<F>(F::new(10.0) * h, h, beta)
        + gl32_func0::<F>(F::new(11.0) * h, h, beta)
        + gl32_func0::<F>(F::new(12.0) * h, h, beta)
        + gl32_func0::<F>(F::new(13.0) * h, h, beta)
        + gl32_func0::<F>(F::new(14.0) * h, h, beta)
        + gl32_func0::<F>(F::new(15.0) * h, h, beta)
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
#[cube]
fn ft_inter_1_subtracted<F: Float>(x: F, beta: F) -> F {
    let csi: F = F::cast_from(1.2599210498948732_f64); // M_CBRT2
    let c = F::new(-0.75) * beta * csi;
    let csi_x = csi * x;
    let d = F::new(1.0) + F::new(3.0) * beta * csi * x * F::ln(csi_x + F::sqrt(csi_x * csi_x + F::new(1.0)));
    // g(x) = C·ln(x)·(1/D - 1) = C·ln(x)·(1 - D)/D
    c * F::ln(x) * (F::new(1.0) - d) / d
}

/// Apply 32-point GL to the subtracted integrand on sub-interval [a, a+h].
#[cube]
fn gl32_subtracted<F: Float>(a: F, h: F, beta: F) -> F {
    let half = h * F::new(0.5);
    let mid = a + half;

    let n01: F = F::cast_from(0.0483076656877383162_f64); let w01: F = F::cast_from(0.0965400885147278006_f64);
    let n02: F = F::cast_from(0.1444719615827964935_f64); let w02: F = F::cast_from(0.0956387200792748594_f64);
    let n03: F = F::cast_from(0.2392873622521370745_f64); let w03: F = F::cast_from(0.0938443990808045656_f64);
    let n04: F = F::cast_from(0.3318686022821276498_f64); let w04: F = F::cast_from(0.0911738786957638847_f64);
    let n05: F = F::cast_from(0.4213512761306353454_f64); let w05: F = F::cast_from(0.0876520930044038111_f64);
    let n06: F = F::cast_from(0.5068999089322293900_f64); let w06: F = F::cast_from(0.0833119242269467552_f64);
    let n07: F = F::cast_from(0.5877157572407623210_f64); let w07: F = F::cast_from(0.0781938957870703065_f64);
    let n08: F = F::cast_from(0.6630442669302152009_f64); let w08: F = F::cast_from(0.0723457941088485062_f64);
    let n09: F = F::cast_from(0.7321821187402896804_f64); let w09: F = F::cast_from(0.0658222227763618468_f64);
    let n10: F = F::cast_from(0.7944837959679424070_f64); let w10: F = F::cast_from(0.0586840934785355471_f64);
    let n11: F = F::cast_from(0.8493676137325699701_f64); let w11: F = F::cast_from(0.0509980592623761762_f64);
    let n12: F = F::cast_from(0.8963211557660521239_f64); let w12: F = F::cast_from(0.0428358980222266807_f64);
    let n13: F = F::cast_from(0.9349060759377396892_f64); let w13: F = F::cast_from(0.0342738629130214331_f64);
    let n14: F = F::cast_from(0.9647622555875064308_f64); let w14: F = F::cast_from(0.0253920653092620595_f64);
    let n15: F = F::cast_from(0.9856115115452683354_f64); let w15: F = F::cast_from(0.0162743947309056706_f64);
    let n16: F = F::cast_from(0.9972638618494815635_f64); let w16: F = F::cast_from(0.0070186100094700966_f64);

    let mut s: F = F::new(0.0);
    s = s + w01 * ft_inter_1_subtracted::<F>(mid + half * n01, beta);
    s = s + w02 * ft_inter_1_subtracted::<F>(mid + half * n02, beta);
    s = s + w03 * ft_inter_1_subtracted::<F>(mid + half * n03, beta);
    s = s + w04 * ft_inter_1_subtracted::<F>(mid + half * n04, beta);
    s = s + w05 * ft_inter_1_subtracted::<F>(mid + half * n05, beta);
    s = s + w06 * ft_inter_1_subtracted::<F>(mid + half * n06, beta);
    s = s + w07 * ft_inter_1_subtracted::<F>(mid + half * n07, beta);
    s = s + w08 * ft_inter_1_subtracted::<F>(mid + half * n08, beta);
    s = s + w09 * ft_inter_1_subtracted::<F>(mid + half * n09, beta);
    s = s + w10 * ft_inter_1_subtracted::<F>(mid + half * n10, beta);
    s = s + w11 * ft_inter_1_subtracted::<F>(mid + half * n11, beta);
    s = s + w12 * ft_inter_1_subtracted::<F>(mid + half * n12, beta);
    s = s + w13 * ft_inter_1_subtracted::<F>(mid + half * n13, beta);
    s = s + w14 * ft_inter_1_subtracted::<F>(mid + half * n14, beta);
    s = s + w15 * ft_inter_1_subtracted::<F>(mid + half * n15, beta);
    s = s + w16 * ft_inter_1_subtracted::<F>(mid + half * n16, beta);
    s = s + w01 * ft_inter_1_subtracted::<F>(mid - half * n01, beta);
    s = s + w02 * ft_inter_1_subtracted::<F>(mid - half * n02, beta);
    s = s + w03 * ft_inter_1_subtracted::<F>(mid - half * n03, beta);
    s = s + w04 * ft_inter_1_subtracted::<F>(mid - half * n04, beta);
    s = s + w05 * ft_inter_1_subtracted::<F>(mid - half * n05, beta);
    s = s + w06 * ft_inter_1_subtracted::<F>(mid - half * n06, beta);
    s = s + w07 * ft_inter_1_subtracted::<F>(mid - half * n07, beta);
    s = s + w08 * ft_inter_1_subtracted::<F>(mid - half * n08, beta);
    s = s + w09 * ft_inter_1_subtracted::<F>(mid - half * n09, beta);
    s = s + w10 * ft_inter_1_subtracted::<F>(mid - half * n10, beta);
    s = s + w11 * ft_inter_1_subtracted::<F>(mid - half * n11, beta);
    s = s + w12 * ft_inter_1_subtracted::<F>(mid - half * n12, beta);
    s = s + w13 * ft_inter_1_subtracted::<F>(mid - half * n13, beta);
    s = s + w14 * ft_inter_1_subtracted::<F>(mid - half * n14, beta);
    s = s + w15 * ft_inter_1_subtracted::<F>(mid - half * n15, beta);
    s = s + w16 * ft_inter_1_subtracted::<F>(mid - half * n16, beta);
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
#[cube]
fn gl32_func1_graded<F: Float>(a: F, h: F, beta: F) -> F {
    let n01: F = F::cast_from(0.0483076656877383162_f64); let w01: F = F::cast_from(0.0965400885147278006_f64);
    let n02: F = F::cast_from(0.1444719615827964935_f64); let w02: F = F::cast_from(0.0956387200792748594_f64);
    let n03: F = F::cast_from(0.2392873622521370745_f64); let w03: F = F::cast_from(0.0938443990808045656_f64);
    let n04: F = F::cast_from(0.3318686022821276498_f64); let w04: F = F::cast_from(0.0911738786957638847_f64);
    let n05: F = F::cast_from(0.4213512761306353454_f64); let w05: F = F::cast_from(0.0876520930044038111_f64);
    let n06: F = F::cast_from(0.5068999089322293900_f64); let w06: F = F::cast_from(0.0833119242269467552_f64);
    let n07: F = F::cast_from(0.5877157572407623210_f64); let w07: F = F::cast_from(0.0781938957870703065_f64);
    let n08: F = F::cast_from(0.6630442669302152009_f64); let w08: F = F::cast_from(0.0723457941088485062_f64);
    let n09: F = F::cast_from(0.7321821187402896804_f64); let w09: F = F::cast_from(0.0658222227763618468_f64);
    let n10: F = F::cast_from(0.7944837959679424070_f64); let w10: F = F::cast_from(0.0586840934785355471_f64);
    let n11: F = F::cast_from(0.8493676137325699701_f64); let w11: F = F::cast_from(0.0509980592623761762_f64);
    let n12: F = F::cast_from(0.8963211557660521239_f64); let w12: F = F::cast_from(0.0428358980222266807_f64);
    let n13: F = F::cast_from(0.9349060759377396892_f64); let w13: F = F::cast_from(0.0342738629130214331_f64);
    let n14: F = F::cast_from(0.9647622555875064308_f64); let w14: F = F::cast_from(0.0253920653092620595_f64);
    let n15: F = F::cast_from(0.9856115115452683354_f64); let w15: F = F::cast_from(0.0162743947309056706_f64);
    let n16: F = F::cast_from(0.9972638618494815635_f64); let w16: F = F::cast_from(0.0070186100094700966_f64);

    let mut s: F = F::new(0.0);

    // For each GL node t ∈ [-1,1]: u = (t+1)/2, x = a + h*u³, jacobian = 3h*u²/2
    let u = (n01 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w01 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n02 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w02 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n03 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w03 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n04 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w04 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n05 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w05 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n06 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w06 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n07 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w07 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n08 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w08 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n09 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w09 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n10 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w10 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n11 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w11 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n12 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w12 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n13 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w13 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n14 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w14 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n15 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w15 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (n16 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w16 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    // Negative nodes
    let u = (-n01 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w01 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n02 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w02 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n03 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w03 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n04 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w04 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n05 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w05 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n06 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w06 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n07 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w07 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n08 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w08 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n09 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w09 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n10 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w10 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n11 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w11 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n12 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w12 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n13 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w13 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n14 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w14 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n15 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w15 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    let u = (-n16 + F::new(1.0)) * F::new(0.5); let x = a + h * u * u * u; s = s + w16 * ft_inter_1::<F>(x, beta) * F::new(1.5) * h * u * u;
    s
}

/// Apply 32-point GL to func1 on regular sub-interval [a, a+h].
#[cube]
fn gl32_func1_regular<F: Float>(a: F, h: F, beta: F) -> F {
    let half = h * F::new(0.5);
    let mid = a + half;

    let n01: F = F::cast_from(0.0483076656877383162_f64); let w01: F = F::cast_from(0.0965400885147278006_f64);
    let n02: F = F::cast_from(0.1444719615827964935_f64); let w02: F = F::cast_from(0.0956387200792748594_f64);
    let n03: F = F::cast_from(0.2392873622521370745_f64); let w03: F = F::cast_from(0.0938443990808045656_f64);
    let n04: F = F::cast_from(0.3318686022821276498_f64); let w04: F = F::cast_from(0.0911738786957638847_f64);
    let n05: F = F::cast_from(0.4213512761306353454_f64); let w05: F = F::cast_from(0.0876520930044038111_f64);
    let n06: F = F::cast_from(0.5068999089322293900_f64); let w06: F = F::cast_from(0.0833119242269467552_f64);
    let n07: F = F::cast_from(0.5877157572407623210_f64); let w07: F = F::cast_from(0.0781938957870703065_f64);
    let n08: F = F::cast_from(0.6630442669302152009_f64); let w08: F = F::cast_from(0.0723457941088485062_f64);
    let n09: F = F::cast_from(0.7321821187402896804_f64); let w09: F = F::cast_from(0.0658222227763618468_f64);
    let n10: F = F::cast_from(0.7944837959679424070_f64); let w10: F = F::cast_from(0.0586840934785355471_f64);
    let n11: F = F::cast_from(0.8493676137325699701_f64); let w11: F = F::cast_from(0.0509980592623761762_f64);
    let n12: F = F::cast_from(0.8963211557660521239_f64); let w12: F = F::cast_from(0.0428358980222266807_f64);
    let n13: F = F::cast_from(0.9349060759377396892_f64); let w13: F = F::cast_from(0.0342738629130214331_f64);
    let n14: F = F::cast_from(0.9647622555875064308_f64); let w14: F = F::cast_from(0.0253920653092620595_f64);
    let n15: F = F::cast_from(0.9856115115452683354_f64); let w15: F = F::cast_from(0.0162743947309056706_f64);
    let n16: F = F::cast_from(0.9972638618494815635_f64); let w16: F = F::cast_from(0.0070186100094700966_f64);

    let mut s: F = F::new(0.0);
    s = s + w01 * ft_inter_1::<F>(mid + half * n01, beta);
    s = s + w02 * ft_inter_1::<F>(mid + half * n02, beta);
    s = s + w03 * ft_inter_1::<F>(mid + half * n03, beta);
    s = s + w04 * ft_inter_1::<F>(mid + half * n04, beta);
    s = s + w05 * ft_inter_1::<F>(mid + half * n05, beta);
    s = s + w06 * ft_inter_1::<F>(mid + half * n06, beta);
    s = s + w07 * ft_inter_1::<F>(mid + half * n07, beta);
    s = s + w08 * ft_inter_1::<F>(mid + half * n08, beta);
    s = s + w09 * ft_inter_1::<F>(mid + half * n09, beta);
    s = s + w10 * ft_inter_1::<F>(mid + half * n10, beta);
    s = s + w11 * ft_inter_1::<F>(mid + half * n11, beta);
    s = s + w12 * ft_inter_1::<F>(mid + half * n12, beta);
    s = s + w13 * ft_inter_1::<F>(mid + half * n13, beta);
    s = s + w14 * ft_inter_1::<F>(mid + half * n14, beta);
    s = s + w15 * ft_inter_1::<F>(mid + half * n15, beta);
    s = s + w16 * ft_inter_1::<F>(mid + half * n16, beta);
    s = s + w01 * ft_inter_1::<F>(mid - half * n01, beta);
    s = s + w02 * ft_inter_1::<F>(mid - half * n02, beta);
    s = s + w03 * ft_inter_1::<F>(mid - half * n03, beta);
    s = s + w04 * ft_inter_1::<F>(mid - half * n04, beta);
    s = s + w05 * ft_inter_1::<F>(mid - half * n05, beta);
    s = s + w06 * ft_inter_1::<F>(mid - half * n06, beta);
    s = s + w07 * ft_inter_1::<F>(mid - half * n07, beta);
    s = s + w08 * ft_inter_1::<F>(mid - half * n08, beta);
    s = s + w09 * ft_inter_1::<F>(mid - half * n09, beta);
    s = s + w10 * ft_inter_1::<F>(mid - half * n10, beta);
    s = s + w11 * ft_inter_1::<F>(mid - half * n11, beta);
    s = s + w12 * ft_inter_1::<F>(mid - half * n12, beta);
    s = s + w13 * ft_inter_1::<F>(mid - half * n13, beta);
    s = s + w14 * ft_inter_1::<F>(mid - half * n14, beta);
    s = s + w15 * ft_inter_1::<F>(mid - half * n15, beta);
    s = s + w16 * ft_inter_1::<F>(mid - half * n16, beta);
    half * s
}

/// Integrate func1 from 0 to b using singularity subtraction + composite GL.
///
/// ∫₀ᵇ func1(x) dx = C·(b·ln(b) - b) + ∫₀ᵇ g(x) dx
///
/// where g(x) = func1(x) - C·ln(x) is smooth everywhere including at x=0.
/// The smooth remainder is integrated with 16-panel composite 32-point GL
/// (512 total quadrature points) for ≤ 10⁻¹² relative error.
#[cube]
pub fn xc_integrate_func1<F: Float>(b: F, beta: F) -> F {
    let csi: F = F::cast_from(1.2599210498948732_f64); // M_CBRT2
    let c = F::new(-0.75) * beta * csi;

    // Exact analytical part: ∫₀ᵇ C·ln(x) dx = C·(b·ln(b) - b)
    let analytical = c * (b * F::ln(b) - b);

    // 32-panel composite GL for the smooth subtracted integrand g(x)
    let h = b / F::new(32.0);
    let smooth =
          gl32_subtracted::<F>( F::new( 0.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 1.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 2.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 3.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 4.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 5.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 6.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 7.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 8.0) * h, h, beta)
        + gl32_subtracted::<F>( F::new( 9.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(10.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(11.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(12.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(13.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(14.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(15.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(16.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(17.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(18.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(19.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(20.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(21.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(22.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(23.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(24.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(25.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(26.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(27.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(28.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(29.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(30.0) * h, h, beta)
        + gl32_subtracted::<F>(F::new(31.0) * h, h, beta);

    analytical + smooth
}
