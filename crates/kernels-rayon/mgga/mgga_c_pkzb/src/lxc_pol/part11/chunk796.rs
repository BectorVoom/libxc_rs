//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 796/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk796(t2295: f64, t3135: f64, t237: f64, t3113: f64, t7930: f64, t7979: f64, t7982: f64, t1201: f64, t881: f64, t2317: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8020 = t2295 * t3135;
    let t8028 = t237 * t3113;
    let t8038 = 0.18541666666666666667e-1_f64 * t7930;
    let t8045 = 0.103295e1_f64 * t7930;
    let t8059 = 0.41678e0_f64 * t7979;
    let t8060 = 0.41678e0_f64 * t7982;
    let t8071 = t1201 * t2295;
    let t8076 = 0.60385e0_f64 * t7930;
    let t8090 = 0.33114e0_f64 * t7979;
    let t8091 = 0.33114e0_f64 * t7982;
    let t8102 = t3113 * t881;
    let t8107 = t1201 * t2317;
    (t8020, t8028, t8038, t8045, t8059, t8060, t8071, t8076, t8090, t8091, t8102, t8107)
}
