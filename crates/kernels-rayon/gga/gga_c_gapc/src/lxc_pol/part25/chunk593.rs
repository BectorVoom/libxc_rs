//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 593/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk593(t3403: f64, t3408: f64, t1084: f64, t2995: f64, t6: f64, t966: f64, t134: f64, t875: f64, t3405: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3409 = t3403 * t3408;
    let t3411 = t1084 * t2995;
    let t3412 = t966 * t6;
    let t3413 = t134 * t875;
    let t3414 = t3412 * t3413;
    let t3415 = t3405 * t3414;
    (t3409, t3411, t3412, t3413, t3414, t3415)
}
