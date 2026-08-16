//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3105/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3105(t12917: f64, t17396: f64, t1260: f64, t17289: f64, t13032: f64, t17524: f64, t17544: f64, t3708: f64, t13051: f64, t56730: f64, t12915: f64, t16771: f64, t247: f64, t5384: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57049 = t17396 * t12917;
    let t57053 = t17289 * t1260;
    let t57056 = t13032 * t17524;
    let t57063 = t3708 * t17544;
    let t57065 = t56730 * t13051;
    let t57070 = t5384 * t247 * t12915 * t16771;
    (t57049, t57053, t57056, t57063, t57065, t57070)
}
