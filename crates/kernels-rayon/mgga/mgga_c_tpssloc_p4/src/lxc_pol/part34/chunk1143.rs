//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1143/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1143(t3886: f64, t6439: f64, t1377: f64, t6347: f64, t28192: f64, t80727: f64, t22892: f64, t7691: f64, t90544: f64, t28200: f64, t6883: f64, t23168: f64, t28288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97608 = t3886 * t6439;
    let t97637 = t1377 * t6347;
    let t97664 = t80727 * t28192;
    let t97732 = t22892 * t90544 * t7691;
    let t97750 = t6883 * t28200;
    let t98117 = t23168 * t28288;
    (t97608, t97637, t97664, t97732, t97750, t98117)
}
