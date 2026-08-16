//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1285/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1285(t23097: f64, t2679: f64, t776: f64, t815: f64, t23061: f64, t6604: f64, t23099: f64, t6605: f64, t9661: f64, t232: f64, t47320: f64, t81779: f64, t81785: f64, t81789: f64, t81795: f64, t81797: f64, t81799: f64, t81801: f64, t81804: f64, t81808: f64, t81810: f64, t81812: f64, t81814: f64, t81819: f64, t81822: f64, t81825: f64, t81829: f64) -> f64 {
    let t81833 = t23097 * t815 * t2679 * t776;
    let t81835 = t23061 * t6604;
    let t81836 = t81835 * t23099;
    let t81839 = t6605 * t815 * t9661;
    let t81843 = t23097 * t815 * t47320 * t232;
    let t81845 = -5.0_f64 / 64.0_f64 * t81779 - 0.12111826828242117256e-2_f64 * t81785 - 0.94875976821229918508e-2_f64 * t81789 - 0.42391393898847410397e-2_f64 * t81795 - 0.84782787797694820794e-2_f64 * t81797 + 7.0_f64 / 48.0_f64 * t81799 - t81801 / 512.0_f64 + t81804 / 256.0_f64 - 119.0_f64 / 2304.0_f64 * t81808 + 7.0_f64 / 768.0_f64 * t81810 - t81812 / 1536.0_f64 + t81814 / 256.0_f64 - t81819 / 256.0_f64 - t81822 / 512.0_f64 + 7.0_f64 / 384.0_f64 * t81825 - 0.25434836339308446237e-1_f64 * t81829 + 0.36335480484726351768e-2_f64 * t81833 - 0.50869672678616892476e-1_f64 * t81836 - 0.20186378047070195427e-3_f64 * t81839 + 0.36335480484726351768e-2_f64 * t81843;
    t81845
}
