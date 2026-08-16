//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 962/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk962(t1583: f64, t17648: f64, t1582: f64, t12168: f64, t5171: f64, t1056: f64, t17426: f64, t3018: f64, t1220: f64, t15008: f64, t15016: f64, t15064: f64, t15083: f64, t15179: f64, t15181: f64, t15200: f64, t15205: f64, t1575: f64, t1579: f64, t1588: f64, t17615: f64, t17619: f64, t17623: f64, t17627: f64, t17635: f64, t17645: f64, t4536: f64, t498: f64, t5229: f64, t5233: f64, t5246: f64, t5474: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17649 = t1583 * t17648;
    let t17650 = t1582 * t17649;
    let t17655 = 6.0_f64 * t12168 * t5171;
    let t17656 = t17426 * t1056;
    let t17658 = 6.0_f64 * t3018 * t17656;
    let t17659 = -4.0_f64 * t5474 * t1575 + t17615 * t498 / 2.0_f64 + 50.0_f64 / 9.0_f64 * t17619 * t1588 + 20000.0_f64 / 27.0_f64 * t17623 * t5246 + 20000.0_f64 / 81.0_f64 * t15064 * t17627 + t15179 / 6.0_f64 + t15181 / 3.0_f64 - 50.0_f64 / 3.0_f64 * t15083 * t5229 + t1220 * t17635 / 6.0_f64 - 8.0_f64 / 3.0_f64 * t15008 * t1579 - t4536 * t5233 + 44.0_f64 / 9.0_f64 * t15016 * t1579 - t17645 + t15200 / 2.0_f64 + 34100.0_f64 / 243.0_f64 * t17650 * t1588 + 44.0_f64 / 9.0_f64 * t15205 - t17655 + t17658;
    (t17649, t17650, t17655, t17656, t17658, t17659)
}
