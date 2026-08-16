//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1758/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1758(t1329: f64, t22797: f64, t3770: f64, t6916: f64, t22754: f64, t22757: f64, t22762: f64, t22767: f64, t22768: f64, t22771: f64, t22774: f64, t22777: f64, t22780: f64, t22785: f64, t22786: f64, t22789: f64, t22795: f64) -> (f64, f64) {
    let t22798 = t22797 * t1329;
    let t22799 = 7.0_f64 / 72.0_f64 * t22798;
    let t22800 = t6916 * t3770;
    let t22802 = -t22754 / 1536.0_f64 - t22757 / 768.0_f64 + t22762 / 768.0_f64 + t22767 - t22768 / 1536.0_f64 - 0.20186378047070195427e-3_f64 * t22771 - 0.20186378047070195427e-3_f64 * t22774 + 0.40372756094140390854e-3_f64 * t22777 + 0.28260929265898273598e-2_f64 * t22780 + t22785 - t22786 / 384.0_f64 - t22789 / 192.0_f64 + 0.40372756094140390854e-3_f64 * t22795 + t22799 - t22800 / 48.0_f64;
    (t22798, t22802)
}
