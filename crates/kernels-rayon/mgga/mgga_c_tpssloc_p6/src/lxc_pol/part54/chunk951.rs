//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 951/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk951(t225: f64, t7192: f64, t7179: f64, t22692: f64, t22717: f64, t22725: f64, t1338: f64, t7191: f64, t22923: f64, t22925: f64, t532: f64, t7216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24082 = t7192 * t225;
    let t24095 = t7179 * t225;
    let t24099 = 0.16449340668482264365e-1_f64 * t22692;
    let t24108 = 0.12793931631041761173e0_f64 * t22717;
    let t24110 = 0.52089578783527170489e-1_f64 * t22725;
    let t24116 = t1338 * t7191;
    let t24156 = 0.12793931631041761173e0_f64 * t22923;
    let t24157 = 0.52089578783527170489e-1_f64 * t22925;
    let t24175 = t532 * t7216;
    (t24082, t24095, t24099, t24108, t24110, t24116, t24156, t24157, t24175)
}
