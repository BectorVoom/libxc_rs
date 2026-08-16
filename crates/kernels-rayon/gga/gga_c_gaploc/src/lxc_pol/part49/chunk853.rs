//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 853/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk853(t3720: f64, t723: f64, t701: f64, t1: f64, t106: f64, t12161: f64, t316: f64, t12206: f64, t783: f64, t835: f64, t325: f64, t1858: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38907 = t3720 * t723;
    let t38912 = t3720 * t701;
    let t38947 = t12161 * t1 * t106 * t316;
    let t38950 = t12206 * t783;
    let t38961 = t835 * t12161;
    let t38974 = t325 * t12161;
    let t39002 = t1858 * t3720;
    (t38907, t38912, t38947, t38950, t38961, t38974, t39002)
}
