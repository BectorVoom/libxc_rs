//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 789/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk789(t1484: f64, t857: f64, t865: f64, t23270: f64, t22986: f64, t23204: f64, t7488: f64, t6562: f64, t23168: f64, t7480: f64, t6547: f64, t7489: f64) -> (f64, f64, f64, f64, f64) {
    let t25191 = t857 * t1484;
    let t25192 = t25191 * t865;
    let t25193 = t23270 * t25192;
    let t25194 = t22986 * t25193;
    let t25205 = t23204 * t7488;
    let t25206 = t6562 * t25205;
    let t25209 = t23168 * t7480;
    let t25211 = t6547 * t7489;
    (t25192, t25194, t25206, t25209, t25211)
}
