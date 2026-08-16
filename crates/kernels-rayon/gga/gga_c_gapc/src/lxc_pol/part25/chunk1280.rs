//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1280/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1280(t11321: f64, t5409: f64, t1036: f64, t1463: f64, t33597: f64, t5462: f64, t9388: f64, t11508: f64, t2993: f64, t5392: f64, t11434: f64, t21049: f64, t3021: f64) -> (f64, f64, f64, f64) {
    let t35217 = t11321 * t5409;
    let t35222 = t5462 * t33597 * t1036 * t1463 * t9388;
    let t35225 = t2993 * t11508 * t5392;
    let t35228 = t11434 * t3021 * t21049;
    (t35217, t35222, t35225, t35228)
}
