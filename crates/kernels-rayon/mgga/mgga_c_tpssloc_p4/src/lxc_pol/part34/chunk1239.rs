//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1239/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1239(t101226: f64, t105773: f64, t106624: f64, t106627: f64, t106651: f64, t106655: f64, t106677: f64, t106690: f64, t106699: f64, t1649: f64, t1877: f64, t2057: f64, t2068: f64, t24191: f64, t24344: f64, t2522: f64, t26563: f64, t26744: f64, t28764: f64, t28774: f64, t28778: f64, t28792: f64, t28795: f64, t29106: f64, t4314: f64, t7656: f64, t7845: f64, t84766: f64) -> f64 {
    let t108616 = 9.0_f64 * t26563 * t106677 + 3.0_f64 * t1877 * t24344 * t106699 + 9.0_f64 * t4314 * t7845 * t28764 - 3.0_f64 / 2.0_f64 * t1877 * t101226 * t7656 + 9.0_f64 / 2.0_f64 * t2522 * t2057 * t106651 + 9.0_f64 / 2.0_f64 * t2522 * t7845 * t28778 - 9.0_f64 / 2.0_f64 * t24191 * t106624 - 3.0_f64 * t1877 * t26744 * t28792 - 9.0_f64 * t26563 * t106690 + 3.0_f64 * t105773 * t2068 - 3.0_f64 * t1877 * t84766 * t106655 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t106627 - 3.0_f64 / 2.0_f64 * t1877 * t26744 * t28795 + 3.0_f64 / 2.0_f64 * t1877 * t29106 * t1649 + 9.0_f64 * t2522 * t7845 * t28774;
    t108616
}
