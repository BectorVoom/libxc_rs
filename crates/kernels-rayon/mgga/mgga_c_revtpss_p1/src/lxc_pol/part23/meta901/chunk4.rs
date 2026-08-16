//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2871/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2871(t14546: f64, t1559: f64, t18677: f64, t40922: f64, t4514: f64, t51578: f64, t51588: f64, t51604: f64, t51615: f64, t62612: f64, t62952: f64, t62961: f64, t62968: f64, t76726: f64, t77120: f64, t820: f64, t879: f64) -> f64 {
    let t77259 = -0.33133632253434461091e-3_f64 * t51578 - t51588 + 0.17073386770573548589e-1_f64 * t40922 - 0.65854491829355115987e0_f64 * t820 * t879 * t77120 + 0.39029762157531132076e-1_f64 * t62952 - t51604 - 0.32927245914677557992e-1_f64 * t62961 - t51615 - 0.29272321618148349057e-1_f64 * t62968 - 0.11853808529283920877e2_f64 * t14546 * t18677 * t76726 - 0.19756347548806534796e1_f64 * t4514 * t62612 * t1559;
    t77259
}
