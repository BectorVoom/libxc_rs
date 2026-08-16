//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1389/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1389(t105727: f64, t106671: f64, t106677: f64, t106686: f64, t106690: f64, t106699: f64, t106706: f64, t106712: f64, t1649: f64, t1877: f64, t1915: f64, t20390: f64, t22959: f64, t23295: f64, t25013: f64, t2522: f64, t28: f64, t28448: f64, t28764: f64, t28778: f64, t28789: f64, t4314: f64, t5966: f64, t6670: f64, t7541: f64, t7656: f64, t87975: f64, t98054: f64) -> f64 {
    let t106716 = 9.0_f64 / 2.0_f64 * t2522 * t7541 * t28778 + 9.0_f64 * t22959 * t106671 + t1877 * t105727 * t28 / 2.0_f64 + 9.0_f64 * t25013 * t106677 + 9.0_f64 * t4314 * t7541 * t28764 + 3.0_f64 * t1877 * t87975 * t28789 - 3.0_f64 / 2.0_f64 * t1877 * t6670 * t106686 - 9.0_f64 * t25013 * t106690 - 3.0_f64 / 2.0_f64 * t1877 * t98054 * t7656 + 3.0_f64 / 2.0_f64 * t1877 * t7541 * t5966 + 3.0_f64 * t1877 * t23295 * t106699 + 3.0_f64 / 2.0_f64 * t1877 * t28448 * t1649 - 9.0_f64 * t22959 * t106706 + t1877 * t1915 * t20390 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t1877 * t6670 * t106712;
    t106716
}
