//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3124/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3124(t1248: f64, t16750: f64, t12915: f64, t17344: f64, t17345: f64, t247: f64, t1260: f64, t44843: f64, t17423: f64, t17426: f64, t11249: f64, t5284: f64) -> (f64, f64, f64, f64, f64) {
    let t57498 = t16750 * t1248;
    let t57508 = t17344 * t247 * t12915 * t17345;
    let t57520 = t44843 * t1260;
    let t57534 = t17426 * t17423;
    let t57536 = t5284 * t11249;
    (t57498, t57508, t57520, t57534, t57536)
}
