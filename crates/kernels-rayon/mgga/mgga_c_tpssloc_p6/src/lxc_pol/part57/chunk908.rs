//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 908/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk908(t2085: f64, t6414: f64, t6387: f64, t225: f64, t29290: f64, t29293: f64, t29287: f64, t23030: f64, t30660: f64, t240: f64, t241: f64, t2627: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t102587 = t2085 * t6414;
    let t102801 = t2085 * t6387;
    let t102917 = t29290 * t225;
    let t102922 = t29293 * t225;
    let t102948 = t29287 * t225;
    let t112676 = 0.52089578783527170489e-1_f64 * t23030 * t30660;
    let t112792 = t812 * t2627 * t240 * t241;
    (t102587, t102801, t102917, t102922, t102948, t112676, t112792)
}
