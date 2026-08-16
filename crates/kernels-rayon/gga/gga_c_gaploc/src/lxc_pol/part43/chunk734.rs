//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 734/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk734(t14498: f64, t14506: f64, t14511: f64, t14515: f64, t13004: f64, t13005: f64, t13234: f64, t13237: f64, t13243: f64, t13245: f64, t13839: f64, t13841: f64, t13951: f64, t14491: f64, t331: f64, t748: f64) -> (f64, f64) {
    let t14517 = t14498 + t14506 + t14511 + t14515;
    let t14519 = t14491 * t331 - t14517 * t748 + t13004 - t13005 - t13234 - t13237 + t13243 + t13245 + 4.0_f64 * t13839 - 2.0_f64 * t13841 - 2.0_f64 * t13951;
    (t14517, t14519)
}
