//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2158/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2158(t11887: f64, t52834: f64, t11913: f64, t11880: f64, t11712: f64, t491: f64, t11605: f64, t1760: f64, t15908: f64, t9467: f64, t9882: f64, t118: f64, t2375: f64, t5151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t53565 = t52834 * t11887;
    let t53592 = t52834 * t11913;
    let t53613 = t52834 * t11880;
    let t53646 = t11712 * t11887 * t491;
    let t53677 = t11605 * t1760;
    let t53777 = t15908 * t9467;
    let t53779 = t15908 * t9882;
    let t53782 = t5151 * t118 * t2375;
    (t53565, t53592, t53613, t53646, t53677, t53777, t53779, t53782)
}
