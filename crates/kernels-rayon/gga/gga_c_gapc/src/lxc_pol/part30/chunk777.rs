//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 777/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk777(t1027: f64, t1781: f64, t144: f64, t2974: f64, t7676: f64, t3094: f64, t1587: f64, t2982: f64, t2980: f64, t1: f64, t8785: f64, t1734: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9235 = t1027 * t1781;
    let t9238 = t7676 * t144 * t2974;
    let t9239 = t3094 * t9238;
    let t9241 = t2982 * t1587;
    let t9242 = t2980 * t9241;
    let t9244 = t8785 * t1;
    let t9245 = t1734 * t9244;
    (t9235, t9239, t9241, t9242, t9244, t9245)
}
