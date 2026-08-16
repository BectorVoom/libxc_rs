//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1106/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1106(t193: f64, t24964: f64, t7021: f64, t89: f64, t28719: f64, t6222: f64, t33966: f64, t4129: f64, t35863: f64, t684: f64, t24976: f64, t6317: f64) -> (f64, f64, f64, f64, f64) {
    let t152834 = t89 * t193 * t24964 * t7021;
    let t152838 = t89 * t193 * t6222 * t28719;
    let t152842 = t89 * t193 * t33966 * t4129;
    let t152844 = t35863 * t684;
    let t152846 = t6317 * t24976 * t152844;
    (t152834, t152838, t152842, t152844, t152846)
}
