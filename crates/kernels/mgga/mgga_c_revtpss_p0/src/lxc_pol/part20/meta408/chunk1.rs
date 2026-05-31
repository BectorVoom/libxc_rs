//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1511/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1511<F: Float>(t1014: F, t11150: F, t1003: F, t11735: F, t221: F, t345: F, t346: F, t624: F, t1050: F, t41: F, t1011: F, t1012: F, t1017: F, t11767: F, t3236: F, t3241: F, t344: F, t348: F, t39443: F, t39449: F, t42716: F, t42719: F, t42721: F, t42724: F, t42727: F, sigma0: F) -> (F, F) {
    let t42731 = t1014 * t11150;
    let t42740 = t1003 * t11735;
    let t42745 = F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t345 * t221 * t624 * t346;
    let t42747 = F::cast_from(1.0_f64) / t41 / t1050;
    let t42748 = sigma0 * t42747;
    let t42752 = F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t42716 + t42719 / F::cast_from(108.0_f64) - F::cast_from(154.0_f64) / F::cast_from(243.0_f64) * t42721 * t1017 + F::cast_from(11.0_f64) / F::cast_from(81.0_f64) * t42724 + t42727 / F::cast_from(36.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3241 * t11767 - t1011 * t1012 * t42731 * t39443 / F::cast_from(12.0_f64) - t1011 * t1012 * t3236 * t39449 / F::cast_from(48.0_f64) - F::cast_from(10.0_f64) / F::cast_from(243.0_f64) * t42740 - t42745 + F::cast_from(1309.0_f64) / F::cast_from(486.0_f64) * t42748 * t344 * t348;
    (t42748, t42752)
}
