//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1387/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1387(t1649: f64, t5544: f64, t20778: f64, t28: f64, t105773: f64, t106618: f64, t106621: f64, t106624: f64, t106627: f64, t106636: f64, t106640: f64, t106647: f64, t1877: f64, t1915: f64, t1969: f64, t22959: f64, t2522: f64, t25358: f64, t25372: f64, t28448: f64, t28771: f64, t28774: f64, t28792: f64, t28795: f64, t4314: f64, t6670: f64, t7541: f64, t7649: f64, t82312: f64, t86736: f64) -> f64 {
    let t106651 = t1649 * t5544;
    let t106655 = t28 * t20778;
    let t106667 = 3.0_f64 * t25372 * t106618 - 9.0_f64 / 2.0_f64 * t22959 * t106621 - 9.0_f64 / 2.0_f64 * t22959 * t106624 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t106627 + 3.0_f64 * t105773 * t1969 + 9.0_f64 * t2522 * t7541 * t28774 - t1877 * t6670 * t106636 / 2.0_f64 + 9.0_f64 * t4314 * t1915 * t106640 + 9.0_f64 / 2.0_f64 * t2522 * t28448 * t7649 + 9.0_f64 / 2.0_f64 * t2522 * t1915 * t106647 + 9.0_f64 / 2.0_f64 * t2522 * t1915 * t106651 - 3.0_f64 * t1877 * t82312 * t106655 - 3.0_f64 / 2.0_f64 * t1877 * t25358 * t28795 - 9.0_f64 * t86736 * t28771 - 3.0_f64 * t1877 * t25358 * t28792;
    t106667
}
