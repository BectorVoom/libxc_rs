//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2808/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2808(t1580: f64, t18316: f64, t689: f64, t14480: f64, t252: f64, t2782: f64, t6071: f64, t11008: f64, t23384: f64, t23404: f64, t2765: f64, t40988: f64, t40998: f64, t4533: f64, t50236: f64, t50245: f64, t50248: f64, t50253: f64, t6048: f64, t61411: f64, t61419: f64, t61422: f64, t61430: f64, t61437: f64, t865: f64) -> f64 {
    let t76020 = t689 * t18316 * t1580;
    let t76026 = t2782 * t252 * t14480 * t6071;
    let t76038 = -0.34697458558045176418e-2_f64 * t61411 - 0.11853808529283920877e2_f64 * t865 * t11008 * t6048 * t4533 - 0.19514881078765566038e-2_f64 * t50236 + 0.16463622957338778997e-1_f64 * t76020 + 0.98781737744032673976e-1_f64 * t61419 - 0.65854491829355115984e-1_f64 * t61422 - 0.32927245914677557992e-1_f64 * t76026 - 0.17073386770573548589e-1_f64 * t40988 + 0.58544643236296698113e-1_f64 * t61430 + 0.19514881078765566038e-2_f64 * t50245 + 0.33133632253434461091e-3_f64 * t50248 - t40998 + 0.58544643236296698114e-1_f64 * t61437 - 0.65854491829355115987e0_f64 * t2765 * t23384 + 0.39512695097613069591e1_f64 * t2765 * t23404 - 0.78059524315062264152e-1_f64 * t50253;
    t76038
}
