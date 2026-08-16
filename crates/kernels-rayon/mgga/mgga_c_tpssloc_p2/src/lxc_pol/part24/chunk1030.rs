//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1030/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1030(t11745: f64, t3506: f64, t11159: f64, t3440: f64, t11168: f64, t1177: f64, t135: f64, t3561: f64, t1174: f64, t11692: f64, t11694: f64, t11699: f64, t11703: f64, t11705: f64, t11709: f64, t11719: f64, t11724: f64, t11728: f64, t11731: f64, t11734: f64, t11738: f64, t11741: f64, t3511: f64, t3518: f64) -> f64 {
    let t11746 = t3506 * t11745;
    let t11748 = t3440 * t11159;
    let t11751 = t1177 * t11168;
    let t11754 = t135 * t3561;
    let t11755 = t1174 * t11754;
    let t11757 = t11692 * t11694 / 1536.0_f64 - t11699 / 1152.0_f64 + t11703 / 1536.0_f64 - t11705 / 1152.0_f64 + t11709 * t3511 / 512.0_f64 + t11719 * t11724 / 512.0_f64 - t11728 * t11731 / 512.0_f64 - t11734 * t3518 / 1024.0_f64 + t11738 * t11741 / 3072.0_f64 + t11746 / 768.0_f64 + t1174 * t11748 / 72.0_f64 - t1174 * t11751 / 48.0_f64 + t11755 / 216.0_f64;
    t11757
}
