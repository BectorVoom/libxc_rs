//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 791/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk791(t144: f64, t2974: f64, t7676: f64, t3094: f64, t1587: f64, t2982: f64, t2980: f64, t1: f64, t8785: f64, t1734: f64, t1030: f64, t2997: f64, t674: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9238 = t7676 * t144 * t2974;
    let t9239 = t3094 * t9238;
    let t9241 = t2982 * t1587;
    let t9242 = t2980 * t9241;
    let t9244 = t8785 * t1;
    let t9245 = t1734 * t9244;
    let t9246 = t1030 * t9245;
    let t9247 = t2997 * t674;
    (t9239, t9241, t9242, t9244, t9245, t9246, t9247)
}
