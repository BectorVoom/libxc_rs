//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1050/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1050(t12153: f64, t972: f64, t1125: f64, t9375: f64, t3449: f64, t3565: f64, t3832: f64, t7056: f64, t11046: f64, t3268: f64, t3265: f64, t3622: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12154 = t12153 * t972;
    let t12155 = t9375 * t1125;
    let t12156 = t3565 * t3449;
    let t12158 = 2.0_f64 * t7056 * t3832;
    let t12161 = 2.0_f64 * t11046 * t3268;
    let t12162 = t3265 * t3622;
    (t12154, t12155, t12156, t12158, t12161, t12162)
}
