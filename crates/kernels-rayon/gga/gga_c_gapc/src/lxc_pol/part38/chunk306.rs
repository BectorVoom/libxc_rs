//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 306/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk306(t345: f64, t13: f64, t30: f64, t1188: f64) -> f64 {
    let t1207 = t345 * t345;
    let t1208 = 1.0_f64 / t1207;
    let t1209 = t13 * t1208;
    let t1210 = t30 * t30;
    let t1211 = 1.0_f64 / t1210;
    let t1212 = t1188 * t1211;
    let t1214 = 0.16081824322151104822e2_f64 * t1209 * t1212;
    t1214
}
