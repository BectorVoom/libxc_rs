//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1255/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1255(t11216: f64, t1448: f64, t4055: f64, t11204: f64, t25127: f64, t11211: f64, t25117: f64, t11227: f64, t8286: f64, t8291: f64, t11202: f64, t128: f64, t15354: f64, t25054: f64) -> (f64, f64, f64, f64, f64) {
    let t35478 = t11216 * t1448 * t4055;
    let t35480 = t11204 * t25127;
    let t35482 = t25117 * t11211;
    let t35485 = t8286 * t11227 * t8291;
    let t35489 = t11202 * t15354 * t128 * t25054;
    (t35478, t35480, t35482, t35485, t35489)
}
