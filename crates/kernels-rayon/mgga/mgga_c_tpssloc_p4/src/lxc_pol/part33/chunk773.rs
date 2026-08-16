//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 773/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk773(t265: f64, t394: f64, t1484: f64, t1915: f64, t202: f64, t7540: f64, t1530: f64, t1877: f64, t193: f64, t2522: f64, t6670: f64, t870: f64, t1070: f64, t1637: f64, t336: f64, t4700: f64, t6822: f64, t7627: f64) -> (f64, f64) {
    let t395 = t265 < t394;
    let t7634 = t1915 * t1484;
    let t7637 = t202 * t7540;
    let t7642 = -t1530 * t1877 * t6670 + t193 * t7637 * t870 + 3.0_f64 * t2522 * t7634;
    let t7643 = piecewise3(t395, t1070 * t193 * t336 * t7627 - t1637 * t4700 * t6822, t7642);
    (t7642, t7643)
}
