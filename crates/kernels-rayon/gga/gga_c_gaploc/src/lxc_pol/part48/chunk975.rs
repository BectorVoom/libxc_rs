//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 975/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk975(t11172: f64, t1445: f64, t2293: f64, t597: f64, t13383: f64, t1580: f64, t11259: f64, t2464: f64, t2465: f64, t6914: f64, t18313: f64, t18372: f64, t44386: f64, t590: f64) -> (f64, f64, f64, f64) {
    let t46471 = 0.43710935587469654631e2_f64 * t597 * t1445 * t11172 * t2293;
    let t46473 = 0.11502877786176224903e2_f64 * t1580 * t13383;
    let t46480 = t6914 * t2464 * t2465 * t11259;
    let t46490 = 0.61348681526273199482e1_f64 * t18372 * t18313 * t44386 * t590;
    (t46471, t46473, t46480, t46490)
}
