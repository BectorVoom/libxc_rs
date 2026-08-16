//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1204/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1204(t193: f64, t201: f64, t7109: f64, t10143: f64, t82069: f64, t2047: f64, t2678: f64, t81598: f64, t81735: f64, t81742: f64, t81724: f64, t81728: f64, t81731: f64, t81738: f64, t81746: f64, t81750: f64, t81752: f64, t81754: f64, t81756: f64, t81758: f64, t81760: f64, t81764: f64, t81767: f64, t81770: f64, t81772: f64, t81774: f64, t81776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t84797 = t193 * t201 * t7109;
    let t84800 = t7109 * t10143;
    let t84820 = 0.19739208802178717238e0_f64 * t82069;
    let t84842 = t2047 * t2678;
    let t84851 = 0.3244175520728446583e0_f64 * t81598;
    let t84857 = 0.13958506597733353653e-1_f64 * t81735;
    let t84859 = 0.87474304870637513515e-3_f64 * t81742;
    let t84873 = t81724 / 128.0_f64 - 0.14534192193890540707e-1_f64 * t81728 + 0.24223653656484234512e-2_f64 * t81731 - t84857 - 0.12111826828242117256e-2_f64 * t81738 + t84859 + 0.72670960969452703536e-2_f64 * t81746 - 7.0_f64 / 48.0_f64 * t81750 + t81752 / 64.0_f64 + t81754 / 64.0_f64 - t81756 / 32.0_f64 - t81758 / 256.0_f64 - t81760 / 64.0_f64 - 119.0_f64 / 288.0_f64 * t81764 - t81767 / 64.0_f64 + 7.0_f64 / 48.0_f64 * t81770 + 7.0_f64 / 96.0_f64 * t81772 - t81774 / 192.0_f64 + 5.0_f64 / 64.0_f64 * t81776;
    (t84797, t84800, t84820, t84842, t84851, t84873)
}
