//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2624/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2624(t40398: f64, t6024: f64, t18435: f64, t221: f64, t10703: f64, t2674: f64, t14832: f64, t2661: f64, t62351: f64, t775: f64, t10716: f64, t18423: f64) -> (f64, f64, f64, f64) {
    let t62401 = t40398 * t6024;
    let t62403 = t221 * t18435;
    let t62405 = t2674 * t10703 * t62403;
    let t62429 = t2661 * t14832 * t62351 * t775;
    let t62431 = t10716 * t18423;
    (t62401, t62405, t62429, t62431)
}
