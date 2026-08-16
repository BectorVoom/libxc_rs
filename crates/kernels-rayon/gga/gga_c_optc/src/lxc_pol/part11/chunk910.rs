//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 910/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk910(t16988: f64, t288: f64, t8197: f64, t8210: f64, t3927: f64, t4772: f64, t2722: f64, t2813: f64, t11192: f64, t11199: f64, t14539: f64, t14600: f64, t17005: f64, t17009: f64, t17018: f64, t17106: f64, t17109: f64, t17115: f64, t2721: f64, t2812: f64, t3884: f64, t8194: f64, t8208: f64, t930: f64, t953: f64) -> (f64, f64, f64) {
    let t17118 = t288 * t16988;
    let t17119 = t17118 * t8197;
    let t17122 = t17118 * t8210;
    let t17125 = t3927 * t4772;
    let t17126 = t2722 * t17125;
    let t17129 = t2813 * t17125;
    let t17133 = -0.19318136643975017455e-1_f64 * t11192 - 0.33587136305576131526e-2_f64 * t11199 + 0.75734008510040627575e0_f64 * t14539 + 0.50380704458364197288e-2_f64 * t953 * t17009 + 0.22391424203717421017e-1_f64 * t953 * t17005 + 0.28977204965962526182e-1_f64 * t930 * t17106 + 0.90151304338550081454e-1_f64 * t930 * t17109 - 0.50380704458364197289e-1_f64 * t953 * t17018 - 0.13186481011862155443e4_f64 * t3884 * t17115 + 0.56690705297447127569e5_f64 * t8194 * t17119 + 0.34014423178468276541e6_f64 * t8208 * t17122 - 0.22720202553012188272e1_f64 * t2721 * t17126 - 0.2339219295794108718e2_f64 * t2812 * t17129 + 0.11360101276506094136e1_f64 * t14600;
    (t17118, t17125, t17133)
}
