//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1301/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1301(t10327: f64, t1890: f64, t4010: f64, t6012: f64, t4006: f64, t10315: f64, t1238: f64, t2028: f64, t20346: f64, t2052: f64, t2055: f64, t2060: f64, t24216: f64, t24218: f64, t24220: f64, t27374: f64, t3023: f64, t3171: f64, t3177: f64, t3938: f64, t457: f64, t572: f64, t6299: f64, t699: f64) -> f64 {
    let t28291 = t1890 * t10327;
    let t28297 = t6012 * t4010;
    let t28299 = t6012 * t4006;
    let t28309 = -4.0_f64 / 81.0_f64 * t24216 + 2.0_f64 / 27.0_f64 * t24218 + 2.0_f64 / 27.0_f64 * t572 * t3171 * t6299 * t3938 * t2028 - t572 * t3177 * t10315 * t2028 / 9.0_f64 + 4.0_f64 / 81.0_f64 * t3023 * t2052 * t2055 * t1238 + t20346 + 142.0_f64 / 243.0_f64 * t24220 + t28291 / 81.0_f64 - 4.0_f64 / 27.0_f64 * t3023 * t699 * t2060 * t1238 - 2.0_f64 / 243.0_f64 * t28297 + 4.0_f64 / 243.0_f64 * t28299 + 8.0_f64 / 27.0_f64 * t27374 * t699 * t2060 * t457 - 8.0_f64 / 81.0_f64 * t27374 * t2052 * t2055 * t457;
    t28309
}
