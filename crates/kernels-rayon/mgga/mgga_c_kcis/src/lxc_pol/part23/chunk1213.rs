//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1213/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1213(t17460: f64, t7948: f64, t97742: f64, t97744: f64, t97746: f64, t97748: f64, t97750: f64, t97752: f64, t97754: f64, t97756: f64, t97758: f64, t97760: f64, t97762: f64, t97765: f64, t97768: f64, t97770: f64, t97773: f64, t97775: f64, t97777: f64) -> (f64, f64) {
    let t97779 = t7948 * t17460;
    let t97781 = 0.17986111111111111111e-1_f64 * t97742 + 0.33333333333333333334e0_f64 * t97744 - 0.21583333333333333334e0_f64 * t97746 - 0.625e-1_f64 * t97748 + 0.20234375e-1_f64 * t97750 + 0.59953703703703703705e-2_f64 * t97752 - 0.4046875e-1_f64 * t97754 - 0.20833333333333333333e-1_f64 * t97756 - 0.10791666666666666667e0_f64 * t97758 + 0.53958333333333333334e-1_f64 * t97760 + 0.53958333333333333334e-1_f64 * t97762 - 0.809375e-1_f64 * t97765 + 0.53958333333333333334e-1_f64 * t97768 - 0.89930555555555555557e-2_f64 * t97770 + 0.125e0_f64 * t97773 - 0.125e0_f64 * t97775 - 0.53958333333333333334e-1_f64 * t97777 - 0.625e-1_f64 * t97779;
    (t97779, t97781)
}
