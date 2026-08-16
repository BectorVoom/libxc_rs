//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 472/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk472(t2380: f64, t2697: f64, t274: f64, t2417: f64, t801: f64, t231: f64, t123: f64, t194: f64, t805: f64, t278: f64, t2014: f64, t2394: f64, t807: f64) -> (f64, f64, f64, f64, f64) {
    let t2698 = t2697 * t2380;
    let t2699 = t2698 * t274;
    let t2701 = t801 * t2417;
    let t2702 = t2701 * t274;
    let t2704 = t2380 * t274;
    let t2705 = t231 * t2704;
    let t2710 = t123 / t805 / t194;
    let t2711 = t2380 * t278;
    let t2719 = -0.11705142615505742e0_f64 * t2699 + 0.23410285231011484e0_f64 * t2702 - 0.26564305359272358183e-2_f64 * t2014 * t2705 + 0.319782988780431561e-1_f64 * t2710 * t2711 - 0.532971647967385935e-1_f64 * t807 * t2417 * t278 + 0.13977476158628290272e-1_f64 * t2394 * t2711;
    (t2698, t2701, t2704, t2710, t2719)
}
