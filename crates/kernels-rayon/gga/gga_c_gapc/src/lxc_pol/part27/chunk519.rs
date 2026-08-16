//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 519/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk519(t2996: f64, t3001: f64, t1030: f64, t2995: f64, t6: f64, t681: f64, t134: f64, t567: f64, t2998: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3002 = t2996 * t3001;
    let t3004 = t1030 * t2995;
    let t3005 = t681 * t6;
    let t3006 = t134 * t567;
    let t3007 = t3005 * t3006;
    let t3008 = t2998 * t3007;
    (t3002, t3004, t3005, t3006, t3007, t3008)
}
