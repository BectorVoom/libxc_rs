//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1378/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1378(t11378: f64, t53566: f64, t14733: f64, t9917: f64, t9923: f64, t2409: f64, t36046: f64, t3965: f64, t12257: f64, t3959: f64, t36007: f64, t53840: f64, t53841: f64, t9872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57657 = t53566 * t11378;
    let t57661 = t14733 * t9917;
    let t57663 = t14733 * t9923;
    let t57666 = t3965 * t2409 * t36046;
    let t57668 = t3959 * t12257;
    let t57671 = t3965 * t2409 * t36007;
    let t57674 = t53840 * t53841 * t9872;
    (t57657, t57661, t57663, t57666, t57668, t57671, t57674)
}
