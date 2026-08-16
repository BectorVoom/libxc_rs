//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1503/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1503(t31463: f64, t575: f64, t1464: f64, t8416: f64, t1455: f64, t8433: f64, t116: f64, t31451: f64, t117338: f64, t118085: f64, t1459: f64, t1461: f64, t1518: f64, t18190: f64, t1916: f64, t1918: f64, t2209: f64, t2327: f64, t2371: f64, t31217: f64, t31234: f64, t31241: f64, t31475: f64, t31497: f64, t31500: f64, t31505: f64, t4158: f64, t4165: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t670: f64, t8336: f64, t8343: f64, t8346: f64, t8406: f64, t8421: f64, t8430: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t118106 = 2.0_f64 * t31463 * t575;
    let t118108 = 2.0_f64 * t8416 * t1464;
    let t118110 = 2.0_f64 * t1455 * t8433;
    let t118137 = t116 * t31451;
    let t118154 = 3.0_f64 * t1916 * t31241 + 6.0_f64 * t572 * t117338 * t1518 + 12.0_f64 * t572 * t31234 * t4292 + 3.0_f64 * t31217 * t1918 + 6.0_f64 * t5795 * t8346 + 6.0_f64 * t572 * t2327 * t8406 + 12.0_f64 * t1459 * t31497 + 6.0_f64 * t31475 * t1461 + 3.0_f64 * t4158 * t8430 + 12.0_f64 * t5795 * t8343 + 12.0_f64 * t572 * t118137 * t670 + 6.0_f64 * t572 * t31505 * t2371 + 12.0_f64 * t1459 * t31500 + 3.0_f64 * t18190 * t2209 + param_d * t118085 * t573 + 12.0_f64 * t8336 * t5802 + 3.0_f64 * t8421 * t4165;
    (t118106, t118108, t118110, t118154)
}
