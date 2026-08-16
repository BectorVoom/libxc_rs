//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1335/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1335(t13796: f64, t13859: f64, t14423: f64, t6220: f64, t13815: f64, t3111: f64, t833: f64, t850: f64, t1123: f64, t50906: f64, t14677: f64, t2397: f64) -> (f64, f64, f64, f64) {
    let t54512 = t13859 * t13796 * t14423 * t6220;
    let t54519 = t850 * t3111 * t13815 * t833;
    let t54523 = t850 * t1123 * t50906 * t833;
    let t54529 = t14677 * t2397;
    (t54512, t54519, t54523, t54529)
}
