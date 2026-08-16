//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 482/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk482(t2380: f64, t278: f64, t2014: f64, t2394: f64, t2417: f64, t2699: f64, t2702: f64, t2705: f64, t2710: f64, t807: f64, t291: f64, t289: f64, t815: f64) -> (f64, f64, f64) {
    let t2711 = t2380 * t278;
    let t2719 = -0.11705142615505742e0_f64 * t2699 + 0.23410285231011484e0_f64 * t2702 - 0.26564305359272358183e-2_f64 * t2014 * t2705 + 0.319782988780431561e-1_f64 * t2710 * t2711 - 0.532971647967385935e-1_f64 * t807 * t2417 * t278 + 0.13977476158628290272e-1_f64 * t2394 * t2711;
    let t2720 = t291 * t2719;
    let t2724 = 1.0_f64 / t815 / t289;
    (t2719, t2720, t2724)
}
