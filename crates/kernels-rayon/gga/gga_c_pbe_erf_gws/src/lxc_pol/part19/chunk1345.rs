//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1345/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1345(t14733: f64, t9923: f64, t2409: f64, t36046: f64, t3965: f64, t12257: f64, t3959: f64, t36007: f64, t53840: f64, t53841: f64, t9872: f64, t12255: f64, t13859: f64, t14797: f64, t3990: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57663 = t14733 * t9923;
    let t57666 = t3965 * t2409 * t36046;
    let t57668 = t3959 * t12257;
    let t57671 = t3965 * t2409 * t36007;
    let t57674 = t53840 * t53841 * t9872;
    let t57678 = t13859 * t3990 * t14797 * t12255;
    (t57663, t57666, t57668, t57671, t57674, t57678)
}
