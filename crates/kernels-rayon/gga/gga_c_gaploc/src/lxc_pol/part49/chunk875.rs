//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 875/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk875(t20561: f64, t2487: f64, t9438: f64, t12531: f64, t6985: f64, t2478: f64, t3137: f64, t6576: f64, t2389: f64, t9302: f64, t12444: f64, t2464: f64, t587: f64) -> (f64, f64, f64, f64, f64) {
    let t40452 = t2487 * t9438 * t20561;
    let t40455 = t2487 * t6985 * t12531;
    let t40458 = t6576 * t3137 * t2478;
    let t40514 = t9302 * t2389;
    let t40517 = t587 * t2464 * t12444;
    (t40452, t40455, t40458, t40514, t40517)
}
