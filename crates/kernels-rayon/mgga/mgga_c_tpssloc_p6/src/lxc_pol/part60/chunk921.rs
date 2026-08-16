//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 921/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk921(t2085: f64, t6387: f64, t225: f64, t29290: f64, t29293: f64, t29287: f64, t111: f64, t29485: f64, t112: f64, t29865: f64, t23030: f64, t30660: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t102801 = t2085 * t6387;
    let t102917 = t29290 * t225;
    let t102922 = t29293 * t225;
    let t102948 = t29287 * t225;
    let t104990 = t29485 * t111;
    let t105105 = t29865 * t112;
    let t112676 = 0.52089578783527170489e-1_f64 * t23030 * t30660;
    (t102801, t102917, t102922, t102948, t104990, t105105, t112676)
}
