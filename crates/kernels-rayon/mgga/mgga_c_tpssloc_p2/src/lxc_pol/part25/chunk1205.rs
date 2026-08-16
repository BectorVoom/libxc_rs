//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1205/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1205(t81779: f64, t81785: f64, t81789: f64, t81795: f64, t81797: f64, t81799: f64, t81801: f64, t81804: f64, t81808: f64, t81810: f64, t81812: f64, t81814: f64, t81819: f64, t81822: f64, t81825: f64, t81829: f64, t81833: f64, t81836: f64, t81839: f64, t81843: f64) -> f64 {
    let t84894 = -5.0_f64 / 32.0_f64 * t81779 - 0.24223653656484234512e-2_f64 * t81785 - 0.18975195364245983701e-1_f64 * t81789 - 0.84782787797694820791e-2_f64 * t81795 - 0.16956557559538964158e-1_f64 * t81797 + 7.0_f64 / 24.0_f64 * t81799 - t81801 / 256.0_f64 + t81804 / 128.0_f64 - 119.0_f64 / 1152.0_f64 * t81808 + 7.0_f64 / 384.0_f64 * t81810 - t81812 / 768.0_f64 + t81814 / 128.0_f64 - t81819 / 128.0_f64 - t81822 / 256.0_f64 + 7.0_f64 / 192.0_f64 * t81825 - 0.50869672678616892475e-1_f64 * t81829 + 0.72670960969452703536e-2_f64 * t81833 - 0.10173934535723378495e0_f64 * t81836 - 0.40372756094140390853e-3_f64 * t81839 + 0.72670960969452703536e-2_f64 * t81843;
    t84894
}
