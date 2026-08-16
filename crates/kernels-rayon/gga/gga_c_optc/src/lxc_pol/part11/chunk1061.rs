//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1061/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1061(t1291: f64, t23163: f64, t1278: f64, t23171: f64, t1264: f64, t2086: f64, t3386: f64, t6642: f64, t6751: f64, t9529: f64, t6739: f64, t6825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29128 = t23163 * t1291;
    let t29210 = t23171 * t1278;
    let t29284 = t1264 * t2086;
    let t29330 = t3386 * t6642;
    let t29335 = t9529 * t6751;
    let t29341 = t9529 * t6739;
    let t29346 = t3386 * t6825;
    (t29128, t29210, t29284, t29330, t29335, t29341, t29346)
}
