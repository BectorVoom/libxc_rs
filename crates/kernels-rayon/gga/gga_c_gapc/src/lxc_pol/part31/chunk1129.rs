//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1129/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1129(t1038: f64, t15489: f64, t9863: f64, t22581: f64, t8676: f64, t1084: f64, t26312: f64, t2536: f64, t2763: f64, t154: f64, t6188: f64, t22783: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28924 = t9863 * t1038 * t15489;
    let t29006 = t8676 * t22581;
    let t29033 = t1084 * t26312;
    let t29070 = t1038 * t2763 * t2536;
    let t29108 = t154 * t6188;
    let t29207 = t8676 * t22783;
    (t28924, t29006, t29033, t29070, t29108, t29207)
}
