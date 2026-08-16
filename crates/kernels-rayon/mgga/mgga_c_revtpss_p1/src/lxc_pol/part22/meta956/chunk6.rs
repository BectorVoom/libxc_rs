//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3206/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3206(t1469: f64, t627: f64, t72: f64, t13389: f64, t13406: f64, t13409: f64, t13414: f64, t1471: f64, t1494: f64, t21686: f64, t21687: f64, t21805: f64, t2251: f64, t2252: f64, t2259: f64, t2260: f64, t2263: f64, t4186: f64, t4188: f64, t4191: f64, t4196: f64, t4238: f64, t5854: f64, t5869: f64, t608: f64, t6977: f64, t85: f64) -> f64 {
    let t60823 = t1469 * t627 * t72;
    let t60829 = -t4196 * t4238 / 3.0_f64 - t2260 * t5869 / 12.0_f64 - t2263 * t5869 / 6.0_f64 - t608 * t21805 / 6.0_f64 - t13406 * t1494 / 6.0_f64 - t13409 * t1494 / 3.0_f64 - t4188 * t4238 / 3.0_f64 - t13414 * t1494 / 6.0_f64 - t4191 * t4238 / 3.0_f64 - t1471 * t13389 / 6.0_f64 - t2259 * t5854 * t85 / 12.0_f64 - t2252 * t5869 / 12.0_f64 - t2251 * t5854 * t85 / 12.0_f64 - t60823 * t21687 / 3.0_f64 - t21686 * t6977 * t4186 / 3.0_f64;
    t60829
}
