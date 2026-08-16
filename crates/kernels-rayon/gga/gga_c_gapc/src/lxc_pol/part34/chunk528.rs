//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 528/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk528(t3053: f64, t649: f64, t128: f64, t654: f64, t185: f64, t122: f64, t424: f64) -> (f64, f64, f64, f64, f64) {
    let t3054 = t3053 * t649;
    let t3056 = t654 * t128;
    let t3057 = t185 * t3056;
    let t3058 = t3057 * t649;
    let t3060 = t424 * t122;
    (t3054, t3056, t3057, t3058, t3060)
}
