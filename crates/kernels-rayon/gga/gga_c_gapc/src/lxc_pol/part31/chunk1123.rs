//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1123/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1123(t103: f64, t134: f64, t22117: f64, t19636: f64, t647: f64, t5056: f64, t172: f64, t6: f64, t674: f64, t1672: f64, t3074: f64, t4: f64, t5972: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t27596 = t134 * t22117 * t103;
    let t27597 = t19636 * t647 * t27596;
    let t27622 = t5056 * pi;
    let t27624 = t6 * t674 * t172;
    let t27658 = t1672 * t3074 * t5972 * t4;
    (t27596, t27597, t27622, t27624, t27658)
}
