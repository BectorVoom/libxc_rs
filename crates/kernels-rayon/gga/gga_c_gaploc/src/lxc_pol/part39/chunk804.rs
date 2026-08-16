//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 804/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk804(t13930: f64, t13948: f64, t12277: f64, t977: f64, t12849: f64, t12858: f64, t12864: f64, t13005: f64, t13232: f64, t13234: f64, t13237: f64, t13239: f64, t13245: f64, t13763: f64, t13764: f64, t13837: f64, t331: f64) -> (f64, f64) {
    let t13949 = t13930 + t13948;
    let t13951 = t12277 * t977;
    let t13952 = t13949 * t331 - t12849 + t12858 - t12864 - t13005 - t13232 - t13234 - t13237 + 2.0_f64 * t13239 + t13245 - t13763 + t13764 + t13837 - t13951;
    (t13949, t13952)
}
