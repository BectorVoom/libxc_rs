//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1292/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1292(t1184: f64, t6199: f64, t2234: f64, t6201: f64, t851: f64, t22233: f64, t18427: f64, t18430: f64, t18433: f64, t18596: f64, t22230: f64, t22236: f64, t22262: f64, t378: f64) -> (f64, f64) {
    let t22684 = t6199 * t1184;
    let t22688 = 0.1551780387578202009e4_f64 * t22684 * t6201 * t2234 * t851;
    let t22693 = 0.37083333333333333334e-1_f64 * t22233;
    let t22697 = (t18596 - 0.86527777777777777777e-1_f64 * t18427 + 0.37083333333333333333e-1_f64 * t18430 - 0.92708333333333333333e-2_f64 * t18433 - 0.28842592592592592592e-1_f64 * t22230 + t22693 - 0.278125e-1_f64 * t22236 + 0.278125e-1_f64 * t22262) * t378;
    (t22688, t22697)
}
