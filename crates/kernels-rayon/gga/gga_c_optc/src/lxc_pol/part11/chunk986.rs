//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 986/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk986(t4477: f64, t5110: f64, t3245: f64, t17907: f64, t914: f64, t17903: f64, t5311: f64, t9073: f64, t4327: f64, t4356: f64, t4458: f64, t17697: f64, t430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18030 = t5110 * t4477;
    let t18031 = t3245 * t18030;
    let t18034 = t914 * t17907;
    let t18037 = t914 * t17903;
    let t18042 = t9073 * t5311;
    let t18043 = t4327 * t18042;
    let t18054 = t4356 * t5311;
    let t18055 = t4458 * t18054;
    let t18058 = t430 * t17697;
    (t18030, t18031, t18034, t18037, t18042, t18043, t18054, t18055, t18058)
}
