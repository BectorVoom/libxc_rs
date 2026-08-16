//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 861/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk861(t2545: f64, t7200: f64, t10008: f64, t320: f64, t9029: f64, t315: f64, t7216: f64, t2664: f64, t9501: f64, t2316: f64, t2636: f64, t3378: f64) -> (f64, f64, f64, f64) {
    let t10009 = t2545 * t7200;
    let t10010 = t10008 * t10009;
    let t10012 = t320 * t9029;
    let t10013 = t315 * t7216;
    let t10014 = t10012 * t10013;
    let t10016 = t9501 * t2664;
    let t10018 = t2636 * t2316;
    let t10019 = t3378 * t10018;
    (t10010, t10014, t10016, t10019)
}
