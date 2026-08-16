//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1160/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1160(t27937: f64, t7032: f64, t111: f64, t28942: f64, t2085: f64, t6414: f64, t1338: f64, t29286: f64, t225: f64, t29290: f64, t29293: f64, t29287: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t102303 = t27937 * t7032;
    let t102386 = t28942 * t111;
    let t102587 = t2085 * t6414;
    let t102798 = t1338 * t29286;
    let t102917 = t29290 * t225;
    let t102922 = t29293 * t225;
    let t102948 = t29287 * t225;
    (t102303, t102386, t102587, t102798, t102917, t102922, t102948)
}
