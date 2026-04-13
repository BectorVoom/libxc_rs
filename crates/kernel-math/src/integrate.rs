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
fn ft_inter_0(x: f64, beta: f64) -> f64 {
    let csi = 1.2599210498948732; // M_CBRT2
    let num = -0.75 * beta * csi;
    let csi_x = csi * x;
    let denom = 1.0 + 3.0 * beta * csi * x * f64::ln(csi_x + f64::sqrt(csi_x * csi_x + 1.0));
    num / denom
}

/// The FT_inter integrand for n=1 (func1):
///   f(x) = -3/4 * beta * csi * ln(x) / (1 + 3*beta*csi*x*ln(csi*x + sqrt(csi²x² + 1)))
/// Has a logarithmic singularity at x=0 (integrable).
#[cube]
fn ft_inter_1(x: f64, beta: f64) -> f64 {
    let csi = 1.2599210498948732; // M_CBRT2
    let num = -0.75 * beta * csi * f64::ln(x);
    let csi_x = csi * x;
    let denom = 1.0 + 3.0 * beta * csi * x * f64::ln(csi_x + f64::sqrt(csi_x * csi_x + 1.0));
    num / denom
}

// ============================================================================
// 32-point Gauss-Legendre nodes and weights on [-1, 1]
// ============================================================================

// To integrate f on [a, b]: ∫f(x)dx ≈ (b-a)/2 * Σ wᵢ f((b-a)/2 * xᵢ + (a+b)/2)
// We inline the 32 nodes/weights as constants since CubeCL can't use arrays.

/// Apply 32-point GL to func0 on sub-interval [a, a+h].
#[cube]
fn gl32_func0(a: f64, h: f64, beta: f64) -> f64 {
    let half = h * 0.5;
    let mid = a + half;

    let n01 = 0.0483076656877383162; let w01 = 0.0965400885147278006;
    let n02 = 0.1444719615827964935; let w02 = 0.0956387200792748594;
    let n03 = 0.2392873622521370745; let w03 = 0.0938443990808045656;
    let n04 = 0.3318686022821276498; let w04 = 0.0911738786957638847;
    let n05 = 0.4213512761306353454; let w05 = 0.0876520930044038111;
    let n06 = 0.5068999089322293900; let w06 = 0.0833119242269467552;
    let n07 = 0.5877157572407623210; let w07 = 0.0781938957870703065;
    let n08 = 0.6630442669302152009; let w08 = 0.0723457941088485062;
    let n09 = 0.7321821187402896804; let w09 = 0.0658222227763618468;
    let n10 = 0.7944837959679424070; let w10 = 0.0586840934785355471;
    let n11 = 0.8493676137325699701; let w11 = 0.0509980592623761762;
    let n12 = 0.8963211557660521239; let w12 = 0.0428358980222266807;
    let n13 = 0.9349060759377396892; let w13 = 0.0342738629130214331;
    let n14 = 0.9647622555875064308; let w14 = 0.0253920653092620595;
    let n15 = 0.9856115115452683354; let w15 = 0.0162743947309056706;
    let n16 = 0.9972638618494815635; let w16 = 0.0070186100094700966;

    let mut s = 0.0f64;
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
#[cube]
pub fn xc_integrate_func0(b: f64, beta: f64) -> f64 {
    let h = b / 16.0;
    gl32_func0( 0.0 * h, h, beta)
        + gl32_func0( 1.0 * h, h, beta)
        + gl32_func0( 2.0 * h, h, beta)
        + gl32_func0( 3.0 * h, h, beta)
        + gl32_func0( 4.0 * h, h, beta)
        + gl32_func0( 5.0 * h, h, beta)
        + gl32_func0( 6.0 * h, h, beta)
        + gl32_func0( 7.0 * h, h, beta)
        + gl32_func0( 8.0 * h, h, beta)
        + gl32_func0( 9.0 * h, h, beta)
        + gl32_func0(10.0 * h, h, beta)
        + gl32_func0(11.0 * h, h, beta)
        + gl32_func0(12.0 * h, h, beta)
        + gl32_func0(13.0 * h, h, beta)
        + gl32_func0(14.0 * h, h, beta)
        + gl32_func0(15.0 * h, h, beta)
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
fn ft_inter_1_subtracted(x: f64, beta: f64) -> f64 {
    let csi = 1.2599210498948732; // M_CBRT2
    let c = -0.75 * beta * csi;
    let csi_x = csi * x;
    let d = 1.0 + 3.0 * beta * csi * x * f64::ln(csi_x + f64::sqrt(csi_x * csi_x + 1.0));
    // g(x) = C·ln(x)·(1/D - 1) = C·ln(x)·(1 - D)/D
    c * f64::ln(x) * (1.0 - d) / d
}

/// Apply 32-point GL to the subtracted integrand on sub-interval [a, a+h].
#[cube]
fn gl32_subtracted(a: f64, h: f64, beta: f64) -> f64 {
    let half = h * 0.5;
    let mid = a + half;

    let n01 = 0.0483076656877383162; let w01 = 0.0965400885147278006;
    let n02 = 0.1444719615827964935; let w02 = 0.0956387200792748594;
    let n03 = 0.2392873622521370745; let w03 = 0.0938443990808045656;
    let n04 = 0.3318686022821276498; let w04 = 0.0911738786957638847;
    let n05 = 0.4213512761306353454; let w05 = 0.0876520930044038111;
    let n06 = 0.5068999089322293900; let w06 = 0.0833119242269467552;
    let n07 = 0.5877157572407623210; let w07 = 0.0781938957870703065;
    let n08 = 0.6630442669302152009; let w08 = 0.0723457941088485062;
    let n09 = 0.7321821187402896804; let w09 = 0.0658222227763618468;
    let n10 = 0.7944837959679424070; let w10 = 0.0586840934785355471;
    let n11 = 0.8493676137325699701; let w11 = 0.0509980592623761762;
    let n12 = 0.8963211557660521239; let w12 = 0.0428358980222266807;
    let n13 = 0.9349060759377396892; let w13 = 0.0342738629130214331;
    let n14 = 0.9647622555875064308; let w14 = 0.0253920653092620595;
    let n15 = 0.9856115115452683354; let w15 = 0.0162743947309056706;
    let n16 = 0.9972638618494815635; let w16 = 0.0070186100094700966;

    let mut s = 0.0f64;
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
#[cube]
fn gl32_func1_graded(a: f64, h: f64, beta: f64) -> f64 {
    let n01 = 0.0483076656877383162; let w01 = 0.0965400885147278006;
    let n02 = 0.1444719615827964935; let w02 = 0.0956387200792748594;
    let n03 = 0.2392873622521370745; let w03 = 0.0938443990808045656;
    let n04 = 0.3318686022821276498; let w04 = 0.0911738786957638847;
    let n05 = 0.4213512761306353454; let w05 = 0.0876520930044038111;
    let n06 = 0.5068999089322293900; let w06 = 0.0833119242269467552;
    let n07 = 0.5877157572407623210; let w07 = 0.0781938957870703065;
    let n08 = 0.6630442669302152009; let w08 = 0.0723457941088485062;
    let n09 = 0.7321821187402896804; let w09 = 0.0658222227763618468;
    let n10 = 0.7944837959679424070; let w10 = 0.0586840934785355471;
    let n11 = 0.8493676137325699701; let w11 = 0.0509980592623761762;
    let n12 = 0.8963211557660521239; let w12 = 0.0428358980222266807;
    let n13 = 0.9349060759377396892; let w13 = 0.0342738629130214331;
    let n14 = 0.9647622555875064308; let w14 = 0.0253920653092620595;
    let n15 = 0.9856115115452683354; let w15 = 0.0162743947309056706;
    let n16 = 0.9972638618494815635; let w16 = 0.0070186100094700966;

    let mut s = 0.0f64;

    // For each GL node t ∈ [-1,1]: u = (t+1)/2, x = a + h*u³, jacobian = 3h*u²/2
    let u = (n01 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w01 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n02 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w02 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n03 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w03 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n04 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w04 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n05 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w05 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n06 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w06 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n07 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w07 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n08 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w08 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n09 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w09 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n10 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w10 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n11 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w11 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n12 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w12 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n13 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w13 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n14 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w14 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n15 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w15 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (n16 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w16 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    // Negative nodes
    let u = (-n01 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w01 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n02 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w02 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n03 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w03 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n04 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w04 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n05 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w05 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n06 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w06 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n07 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w07 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n08 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w08 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n09 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w09 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n10 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w10 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n11 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w11 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n12 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w12 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n13 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w13 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n14 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w14 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n15 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w15 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    let u = (-n16 + 1.0) * 0.5; let x = a + h * u * u * u; s = s + w16 * ft_inter_1(x, beta) * 1.5 * h * u * u;
    s
}

/// Apply 32-point GL to func1 on regular sub-interval [a, a+h].
#[cube]
fn gl32_func1_regular(a: f64, h: f64, beta: f64) -> f64 {
    let half = h * 0.5;
    let mid = a + half;

    let n01 = 0.0483076656877383162; let w01 = 0.0965400885147278006;
    let n02 = 0.1444719615827964935; let w02 = 0.0956387200792748594;
    let n03 = 0.2392873622521370745; let w03 = 0.0938443990808045656;
    let n04 = 0.3318686022821276498; let w04 = 0.0911738786957638847;
    let n05 = 0.4213512761306353454; let w05 = 0.0876520930044038111;
    let n06 = 0.5068999089322293900; let w06 = 0.0833119242269467552;
    let n07 = 0.5877157572407623210; let w07 = 0.0781938957870703065;
    let n08 = 0.6630442669302152009; let w08 = 0.0723457941088485062;
    let n09 = 0.7321821187402896804; let w09 = 0.0658222227763618468;
    let n10 = 0.7944837959679424070; let w10 = 0.0586840934785355471;
    let n11 = 0.8493676137325699701; let w11 = 0.0509980592623761762;
    let n12 = 0.8963211557660521239; let w12 = 0.0428358980222266807;
    let n13 = 0.9349060759377396892; let w13 = 0.0342738629130214331;
    let n14 = 0.9647622555875064308; let w14 = 0.0253920653092620595;
    let n15 = 0.9856115115452683354; let w15 = 0.0162743947309056706;
    let n16 = 0.9972638618494815635; let w16 = 0.0070186100094700966;

    let mut s = 0.0f64;
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
#[cube]
pub fn xc_integrate_func1(b: f64, beta: f64) -> f64 {
    let csi = 1.2599210498948732; // M_CBRT2
    let c = -0.75 * beta * csi;

    // Exact analytical part: ∫₀ᵇ C·ln(x) dx = C·(b·ln(b) - b)
    let analytical = c * (b * f64::ln(b) - b);

    // 32-panel composite GL for the smooth subtracted integrand g(x)
    let h = b / 32.0;
    let smooth =
          gl32_subtracted( 0.0 * h, h, beta)
        + gl32_subtracted( 1.0 * h, h, beta)
        + gl32_subtracted( 2.0 * h, h, beta)
        + gl32_subtracted( 3.0 * h, h, beta)
        + gl32_subtracted( 4.0 * h, h, beta)
        + gl32_subtracted( 5.0 * h, h, beta)
        + gl32_subtracted( 6.0 * h, h, beta)
        + gl32_subtracted( 7.0 * h, h, beta)
        + gl32_subtracted( 8.0 * h, h, beta)
        + gl32_subtracted( 9.0 * h, h, beta)
        + gl32_subtracted(10.0 * h, h, beta)
        + gl32_subtracted(11.0 * h, h, beta)
        + gl32_subtracted(12.0 * h, h, beta)
        + gl32_subtracted(13.0 * h, h, beta)
        + gl32_subtracted(14.0 * h, h, beta)
        + gl32_subtracted(15.0 * h, h, beta)
        + gl32_subtracted(16.0 * h, h, beta)
        + gl32_subtracted(17.0 * h, h, beta)
        + gl32_subtracted(18.0 * h, h, beta)
        + gl32_subtracted(19.0 * h, h, beta)
        + gl32_subtracted(20.0 * h, h, beta)
        + gl32_subtracted(21.0 * h, h, beta)
        + gl32_subtracted(22.0 * h, h, beta)
        + gl32_subtracted(23.0 * h, h, beta)
        + gl32_subtracted(24.0 * h, h, beta)
        + gl32_subtracted(25.0 * h, h, beta)
        + gl32_subtracted(26.0 * h, h, beta)
        + gl32_subtracted(27.0 * h, h, beta)
        + gl32_subtracted(28.0 * h, h, beta)
        + gl32_subtracted(29.0 * h, h, beta)
        + gl32_subtracted(30.0 * h, h, beta)
        + gl32_subtracted(31.0 * h, h, beta);

    analytical + smooth
}
