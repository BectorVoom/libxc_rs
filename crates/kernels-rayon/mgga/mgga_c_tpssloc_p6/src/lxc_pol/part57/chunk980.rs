//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 980/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk980(t127630: f64, t8657: f64, t127601: f64, t127603: f64, t127606: f64, t127669: f64, t127671: f64, t127673: f64, t127677: f64, t127679: f64, t127681: f64, t127684: f64, t127686: f64, t127688: f64, t26523: f64, t31795: f64, t5493: f64, t7801: f64, t7956: f64, t8508: f64, t86647: f64) -> f64 {
    let t127690 = 54.0_f64 * t127630 * t8657;
    let t127695 = t8508 + t127669 + t127671 + t127673 + t127601 + t127603 + 54.0_f64 * t86647 * t7956 + t127677 + t127679 + t127681 + t127684 + t127606 + t127686 + t127688 + t127690 + 27.0_f64 * t26523 * t7801 + 0.135e2_f64 * t31795 * t5493;
    t127695
}
