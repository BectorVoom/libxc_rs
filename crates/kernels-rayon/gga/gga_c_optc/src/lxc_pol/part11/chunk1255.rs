//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1255/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1255(t25121: f64, t55901: f64, t14294: f64, t16917: f64, t123: f64, t4561: f64, t24468: f64, t55912: f64, t894: f64, t897: f64, t1235: f64, t49244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56718 = t25121 * t55901;
    let t56722 = t14294 * t16917;
    let t56726 = t4561 * t123;
    let t56727 = t24468 * t56726;
    let t56732 = t894 * t897 * t55912;
    let t56735 = t49244 * t1235;
    (t56718, t56722, t56726, t56727, t56732, t56735)
}
