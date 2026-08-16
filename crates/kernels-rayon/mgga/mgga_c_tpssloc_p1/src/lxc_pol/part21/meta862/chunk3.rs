//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3132/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3132(t3446: f64, t61064: f64, t1176: f64, t1714: f64, t1184: f64, t15293: f64, t15382: f64, t3439: f64, t44424: f64, t44439: f64, t52074: f64, t52076: f64, t52081: f64, t52084: f64, t52086: f64, t52089: f64, t52092: f64, t52109: f64) -> f64 {
    let t64824 = t3446 * t61064;
    let t64825 = t1176 * t1714;
    let t64845 = 0.22222222222222222222e-2_f64 * t64824 * t64825 * t1184 * t15293 + 0.18518518518518518518e-3_f64 * t44424 + 0.18518518518518518518e-3_f64 * t44439 - 0.19753086419753086419e-2_f64 * t52074 + 0.14814814814814814814e-2_f64 * t52076 - 0.6172839506172839506e-3_f64 * t52081 + 0.74074074074074074072e-3_f64 * t52084 + 0.14814814814814814814e-2_f64 * t52086 - 0.14814814814814814814e-2_f64 * t64824 * t3439 * t1714 * t1184 * t15382 + 0.37037037037037037036e-3_f64 * t52089 - 0.74074074074074074072e-3_f64 * t52092 - 0.32921810699588477366e-3_f64 * t52109;
    t64845
}
