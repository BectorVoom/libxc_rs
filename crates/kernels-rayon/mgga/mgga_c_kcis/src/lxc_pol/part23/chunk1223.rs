//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1223/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1223(t97742: f64, t97744: f64, t97746: f64, t97748: f64, t97750: f64, t97752: f64, t97754: f64, t97756: f64, t97758: f64, t97760: f64, t97762: f64, t97765: f64, t97768: f64, t97770: f64, t97773: f64, t97775: f64, t97777: f64, t97779: f64) -> f64 {
    let t97939 = t97742 / 144.0_f64 + 2.0_f64 / 9.0_f64 * t97744 - t97746 / 12.0_f64 - t97748 / 24.0_f64 + t97750 / 128.0_f64 + t97752 / 432.0_f64 - t97754 / 64.0_f64 - t97756 / 72.0_f64 - t97758 / 24.0_f64 + t97760 / 48.0_f64 + t97762 / 48.0_f64 - t97765 / 32.0_f64 + t97768 / 48.0_f64 - t97770 / 288.0_f64 + t97773 / 12.0_f64 - t97775 / 12.0_f64 - t97777 / 48.0_f64 - t97779 / 24.0_f64;
    t97939
}
