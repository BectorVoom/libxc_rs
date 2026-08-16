//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 943/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk943(t2709: f64, t2712: f64, t2716: f64, t2736: f64, t2978: f64, t2982: f64, t2988: f64, t2805: f64, t2811: f64, t2994: f64, t1364: f64, t228: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14717 = 0.28493333333333333333e0_f64 * t2709;
    let t14718 = 0.2137e0_f64 * t2712;
    let t14719 = 0.34367190188705947438e1_f64 * t2716;
    let t14720 = 0.4274e0_f64 * t2736;
    let t14725 = 480.0_f64 * t2978;
    let t14726 = 0.23392894490538584828e1_f64 * t2982;
    let t14729 = 0.14035736694323150897e2_f64 * t2988;
    let t14731 = 0.2069040516770936012e4_f64 * t2805;
    let t14732 = 0.3859675079686208416e3_f64 * t2811;
    let t14734 = 240.0_f64 * t2994;
    let t14810 = 32.0_f64 * t1364 * t228;
    (t14717, t14718, t14719, t14720, t14725, t14726, t14729, t14731, t14732, t14734, t14810)
}
