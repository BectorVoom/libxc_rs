//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 903/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk903(t604: f64, t2464: f64, t8518: f64, t5015: f64, t28532: f64, t1783: f64, t1310: f64, t2430: f64, t4957: f64, t23450: f64, t10949: f64, t1224: f64, t28369: f64) -> (f64, f64, f64, f64) {
    let t659 = 0.0_f64 < t604;
    let t29054 = t8518 * t2464;
    let t29055 = t5015 * t29054;
    let t29059 = piecewise3(t659, t28532, -t28532);
    let t29060 = t1783 * t29059;
    let t29061 = t1310 * t29060;
    let t29073 = t4957 * t2430;
    let t29074 = t23450 * t29073;
    let t29082 = t1224 * t10949 * t28369;
    (t29055, t29061, t29074, t29082)
}
