//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1398/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1398(t1845: f64, t6330: f64, t24995: f64, t8643: f64, t28239: f64, t7685: f64, t22633: f64, t22635: f64, t26337: f64, t6460: f64, t1985: f64, t7700: f64, t97511: f64) -> (f64, f64, f64, f64) {
    let t106971 = t6330 * t1845;
    let t106974 = 18.0_f64 * t24995 * t8643 * t106971;
    let t106978 = 3.0_f64 * t7685 * t28239;
    let t106982 = t22633 * t22635 * t26337 * t6460;
    let t106986 = t1985 * t97511 * t7700;
    (t106974, t106978, t106982, t106986)
}
