//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 910/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk910<F: Float>(t16988: F, t288: F, t8197: F, t8210: F, t3927: F, t4772: F, t2722: F, t2813: F, t11192: F, t11199: F, t14539: F, t14600: F, t17005: F, t17009: F, t17018: F, t17106: F, t17109: F, t17115: F, t2721: F, t2812: F, t3884: F, t8194: F, t8208: F, t930: F, t953: F) -> (F, F, F) {
    let t17118 = t288 * t16988;
    let t17119 = t17118 * t8197;
    let t17122 = t17118 * t8210;
    let t17125 = t3927 * t4772;
    let t17126 = t2722 * t17125;
    let t17129 = t2813 * t17125;
    let t17133 = -F::cast_from(0.19318136643975017455e-1_f64) * t11192 - F::cast_from(0.33587136305576131526e-2_f64) * t11199 + F::cast_from(0.75734008510040627575e0_f64) * t14539 + F::cast_from(0.50380704458364197288e-2_f64) * t953 * t17009 + F::cast_from(0.22391424203717421017e-1_f64) * t953 * t17005 + F::cast_from(0.28977204965962526182e-1_f64) * t930 * t17106 + F::cast_from(0.90151304338550081454e-1_f64) * t930 * t17109 - F::cast_from(0.50380704458364197289e-1_f64) * t953 * t17018 - F::cast_from(0.13186481011862155443e4_f64) * t3884 * t17115 + F::cast_from(0.56690705297447127569e5_f64) * t8194 * t17119 + F::cast_from(0.34014423178468276541e6_f64) * t8208 * t17122 - F::cast_from(0.22720202553012188272e1_f64) * t2721 * t17126 - F::cast_from(0.2339219295794108718e2_f64) * t2812 * t17129 + F::cast_from(0.11360101276506094136e1_f64) * t14600;
    (t17118, t17125, t17133)
}
