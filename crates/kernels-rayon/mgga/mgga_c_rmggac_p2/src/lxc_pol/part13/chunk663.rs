//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 663/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk663(t7599: f64, t8743: f64, t27: f64, t3839: f64, t1635: f64, t649: f64, t3826: f64, t1624: f64, t1627: f64, t7603: f64, t3819: f64, t3851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8744 = t7599 * t8743;
    let t8746 = t3839 * t27;
    let t8747 = t649 * t1635;
    let t8748 = t8746 * t8747;
    let t8750 = t3826 * t27;
    let t8751 = t649 * t1624;
    let t8752 = t8750 * t8751;
    let t8754 = t649 * t1627;
    let t8755 = t7603 * t8754;
    let t8759 = t7603 * t8743;
    let t8761 = t3819 * t27;
    let t8762 = t8761 * t8747;
    let t8764 = t3851 * t27;
    (t8744, t8746, t8748, t8750, t8751, t8752, t8754, t8755, t8759, t8761, t8762, t8764)
}
