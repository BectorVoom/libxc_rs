//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1227/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1227(t1100: f64, t11862: f64, t354: f64, t39181: f64, t39186: f64, t39188: f64, t39195: f64, t39201: f64, t39205: f64, t39208: f64, t39212: f64, t39246: f64, t39267: f64, t39271: f64, t39272: f64, t39276: f64, t39278: f64, t39282: f64, t39322: f64, t40272: f64, t40321: f64, t40355: f64, t40401: f64, t40437: f64, t40476: f64, t40506: f64, t40533: f64, t40583: f64, t40616: f64, t40655: f64, t40695: f64, t40718: f64, t8306: f64, t860: f64) -> f64 {
    let t40724 = t39181 + t39186 + t39188 + t39195 - t39201 - t39205 + t1100 * t8306 - t39208 + t39212 + 2.0_f64 * t860 * t11862 + t354 * (t39246 + t39272 + t39322 + t40272 + t40321 + t40355 + t40401 + t40437 + t40476 + t40506 + t40533 + t40583 + t40616 + t40655 + t40695 + t40718) - t39267 + t39271 + t39276 - t39278 + t39282;
    t40724
}
