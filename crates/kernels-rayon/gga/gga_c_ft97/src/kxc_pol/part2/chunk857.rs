//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 857/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk857(t13346: f64, t3917: f64, t13293: f64, t13297: f64, t13302: f64, t13306: f64, t13308: f64, t13310: f64, t13316: f64, t13321: f64, t13325: f64, t13329: f64, t13332: f64, t13335: f64, t13338: f64, t13339: f64, t13345: f64, t3139: f64, t462: f64, t9905: f64, t9933: f64, t9936: f64, t9962: f64) -> f64 {
    let t13347 = t3917 * t13346;
    let t13350 = -4.0_f64 / 3.0_f64 * t3139 * t13293 + 2.0_f64 / 3.0_f64 * t462 * t13297 - 8.0_f64 / 3.0_f64 * t3139 * t13302 - t13306 + t13308 - 2.0_f64 / 9.0_f64 * t462 * t13310 - 10.0_f64 / 27.0_f64 * t462 * t13316 + 8.0_f64 / 9.0_f64 * t3139 * t13321 + t462 * t13325 / 3.0_f64 - t13329 - 2.0_f64 / 9.0_f64 * t9905 + 2.0_f64 * t462 * t13332 - 4.0_f64 / 27.0_f64 * t13335 - t13338 - 22.0_f64 / 9.0_f64 * t13339 + t9933 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t9936 - 2.0_f64 / 3.0_f64 * t9962 + t13345 - 2.0_f64 * t462 * t13347;
    t13350
}
