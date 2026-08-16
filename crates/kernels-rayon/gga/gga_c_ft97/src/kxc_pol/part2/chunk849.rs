//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 849/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk849(t3677: f64, t7742: f64, t2305: f64, t920: f64, t1080: f64, t13256: f64, t13260: f64, t13263: f64, t13268: f64, t13273: f64, t2240: f64, t2301: f64, t2309: f64, t3601: f64, t3665: f64, t3674: f64, t3678: f64, t623: f64, t650: f64, t8614: f64) -> f64 {
    let t13276 = t3677 * t7742;
    let t13279 = t2305 * t920;
    let t13289 = t623 * t13256 / 2.0_f64 + t623 * t13260 / 4.0_f64 + t623 * t13263 / 4.0_f64 + t2240 * t3665 / 2.0_f64 + t623 * t13268 / 4.0_f64 + t8614 * t1080 / 4.0_f64 + t13273 * t650 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t623 * t13276 + t623 * t13279 / 4.0_f64 + t3601 * t2301 / 4.0_f64 + t3601 * t2309 / 2.0_f64 + t2240 * t3674 / 2.0_f64 + t2240 * t3678;
    t13289
}
