//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 964/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk964(t11222: f64, t200: f64, t1954: f64, t1006: f64, t128: f64, t4864: f64, t11202: f64, t8291: f64, t3640: f64, t518: f64, t3650: f64, t4015: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11223 = t11222 * t200;
    let t11224 = t11223 * t1954;
    let t11225 = t1006 * t11224;
    let t11227 = t4864 * t128;
    let t11228 = t11202 * t11227;
    let t11229 = t11228 * t8291;
    let t11231 = t518 * t3640;
    let t11234 = t3650 * t4015;
    (t11223, t11224, t11225, t11227, t11228, t11229, t11231, t11234)
}
