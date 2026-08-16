//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1045/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1045(t6999: f64, t7217: f64, t22754: f64, t22757: f64, t22762: f64, t22766: f64, t22768: f64, t22771: f64, t22774: f64, t22777: f64, t22780: f64, t22784: f64, t22786: f64, t22789: f64, t22795: f64, t22798: f64, t22800: f64) -> (f64, f64) {
    let t24028 = t7217 * t6999;
    let t24046 = -t22754 / 768.0_f64 - t22757 / 384.0_f64 + t22762 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t22766 - t22768 / 768.0_f64 - 0.40372756094140390853e-3_f64 * t22771 - 0.40372756094140390853e-3_f64 * t22774 + 0.80745512188280781706e-3_f64 * t22777 + 0.56521858531796547194e-2_f64 * t22780 + 7.0_f64 / 144.0_f64 * t22784 - t22786 / 192.0_f64 - t22789 / 96.0_f64 + 0.80745512188280781706e-3_f64 * t22795 + 7.0_f64 / 36.0_f64 * t22798 - t22800 / 24.0_f64;
    (t24028, t24046)
}
