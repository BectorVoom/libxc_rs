//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 258/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk258(t230: f64, t900: f64, t326: f64, t2400: f64, t14: f64, t1576: f64, t17: f64, t12: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2923 = t230 * t900;
    let t2937 = t326 * t326;
    let t2938 = 1.0_f64 / t2937;
    let t2946 = 0.19257444444444444444e0_f64 * t2400;
    let t2998 = 1.0_f64 / t14 / t1576;
    let t2999 = t2998 * t17;
    let t3050 = t12 * t2998;
    let t3051 = t9 * t3050;
    (t2923, t2938, t2946, t2998, t2999, t3050, t3051)
}
