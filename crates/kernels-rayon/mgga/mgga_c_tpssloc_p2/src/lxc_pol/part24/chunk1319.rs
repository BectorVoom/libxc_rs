//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1319/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1319(t23127: f64, t2703: f64, t81724: f64, t81728: f64, t81731: f64, t81736: f64, t81738: f64, t81743: f64, t81746: f64, t81750: f64, t81752: f64, t81754: f64, t81756: f64, t81758: f64, t81760: f64, t81764: f64, t81767: f64, t81770: f64, t81772: f64, t81774: f64) -> f64 {
    let t81776 = t23127 * t2703;
    let t81778 = t81724 / 256.0_f64 - 0.72670960969452703536e-2_f64 * t81728 + 0.12111826828242117256e-2_f64 * t81731 - t81736 - 0.60559134141210586281e-3_f64 * t81738 + t81743 + 0.36335480484726351768e-2_f64 * t81746 - 7.0_f64 / 96.0_f64 * t81750 + t81752 / 128.0_f64 + t81754 / 128.0_f64 - t81756 / 64.0_f64 - t81758 / 512.0_f64 - t81760 / 128.0_f64 - 119.0_f64 / 576.0_f64 * t81764 - t81767 / 128.0_f64 + 7.0_f64 / 96.0_f64 * t81770 + 7.0_f64 / 192.0_f64 * t81772 - t81774 / 384.0_f64 + 5.0_f64 / 128.0_f64 * t81776;
    t81778
}
