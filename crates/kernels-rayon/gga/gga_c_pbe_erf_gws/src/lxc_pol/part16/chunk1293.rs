//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1293/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1293(t13972: f64, t14684: f64, t14767: f64, t2397: f64, t1134: f64, t13776: f64, t2410: f64, t50956: f64, t3959: f64, t8756: f64, t14608: f64, t22393: f64, t2409: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54463 = t13972 * t14684;
    let t54465 = t14767 * t2397;
    let t54473 = t13776 * t50956 * t1134 * t2410;
    let t54484 = t3959 * t8756;
    let t54491 = t13972 * t14608;
    let t54496 = t3959 * t2409 * t22393;
    (t54463, t54465, t54473, t54484, t54491, t54496)
}
