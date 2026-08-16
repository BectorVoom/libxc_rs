//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1010/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1010(t10893: f64, t10898: f64, t10936: f64, t13798: f64, t13801: f64, t13805: f64, t13974: f64, t15304: f64, t15307: f64, t15310: f64, t15317: f64, t15320: f64, t15323: f64, t3550: f64, t3575: f64, t3586: f64, t3592: f64, t5216: f64, t5238: f64) -> f64 {
    let t15326 = -0.19751789702565206229e-1_f64 * t13974 + t13798 + t13801 - t13805 - 0.11696446794910408142e1_f64 * t15304 * t3586 + 6.0_f64 * t3575 * t15307 + 0.35089340384731224426e1_f64 * t3592 * t15310 - 4.0_f64 * t10936 * t5216 + 0.64329366355741395948e2_f64 * t10893 * t5238 - 4.0_f64 * t3550 * t15317 - 2.0_f64 * t3550 * t15320 - 0.19298809906722418785e3_f64 * t10898 * t15323;
    t15326
}
