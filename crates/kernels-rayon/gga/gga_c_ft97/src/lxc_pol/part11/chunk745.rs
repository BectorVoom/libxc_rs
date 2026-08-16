//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 745/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk745(t10080: f64, t684: f64, t10079: f64, t2492: f64, t754: f64, t2602: f64, t10026: f64, t10031: f64, t10036: f64, t10041: f64, t10046: f64, t10048: f64, t10055: f64, t10059: f64, t10062: f64, t10064: f64, t10067: f64, t10071: f64, t10076: f64, t1901: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t10081 = t10080 * t684;
    let t10082 = t10079 * t10081;
    let t10085 = t2492 * t754;
    let t10086 = t10085 * t2602;
    let t10089 = -10.0_f64 / 81.0_f64 * t446 * t10026 - 2.0_f64 * t446 * t10031 - 2.0_f64 * t446 * t10036 + 2.0_f64 * t446 * t10041 + t446 * t10046 + t10048 / 3.0_f64 - 2.0_f64 * t446 * t10055 + 2.0_f64 * t446 * t10059 - 2.0_f64 / 3.0_f64 * t10062 - 2.0_f64 / 3.0_f64 * t10064 - t446 * t10067 + t1901 * t10071 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t10076 - 2.0_f64 / 3.0_f64 * t1901 * t10082 + 2.0_f64 / 3.0_f64 * t1901 * t10086;
    (t10081, t10082, t10085, t10086, t10089)
}
