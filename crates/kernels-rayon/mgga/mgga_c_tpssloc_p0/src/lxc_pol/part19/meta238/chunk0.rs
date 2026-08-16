//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 968/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk968(t11189: f64, t409: f64, t1117: f64, t3265: f64, t3315: f64, t11135: f64, t1102: f64, t3270: f64, t3279: f64, t3287: f64, t10292: f64, t281: f64, t415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11190 = t409 * t11189;
    let t11191 = t3265 * t1117;
    let t11192 = t11191 * t3315;
    let t11194 = 0.96491876992155210402e2_f64 * t11190 * t11192;
    let t11195 = 0.93011851851851851854e0_f64 * t11135;
    let t11197 = t3270 * t1102 * t3279;
    let t11200 = t3287 * t1102 * t3279;
    let t11203 = t281 * t10292 * t415;
    (t11190, t11191, t11192, t11194, t11195, t11197, t11200, t11203)
}
