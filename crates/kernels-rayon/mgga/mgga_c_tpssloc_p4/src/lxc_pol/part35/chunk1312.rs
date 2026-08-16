//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1312/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1312(t1361: f64, t22690: f64, t6330: f64, t80840: f64, t22792: f64, t6347: f64, t22804: f64, t28077: f64, t22779: f64, t28067: f64, t28060: f64, t22892: f64, t22893: f64, t28138: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97427 = t80840 * t22690 * t1361 * t6330;
    let t97431 = t22792 * t22690 * t1361 * t6347;
    let t97439 = t22804 * t28077;
    let t97444 = t22779 * t28067;
    let t97463 = t22779 * t28060;
    let t97494 = t22892 * t22893 * t28138;
    (t97427, t97431, t97439, t97444, t97463, t97494)
}
