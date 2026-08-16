//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1063/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1063(t134: f64, t966: f64, t8133: f64, t7938: f64, t8676: f64, t26995: f64, t7200: f64, t7453: f64, t1045: f64, t818: f64, t332: f64, t7877: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28191 = t966 * t134;
    let t28192 = t28191 * t8133;
    let t28254 = t8676 * t7938;
    let t28346 = t26995 * t7200;
    let t28353 = t26995 * t7453;
    let t28370 = t1045 * t818;
    let t28415 = t332 * t134;
    let t28416 = t28415 * t7877;
    (t28192, t28254, t28346, t28353, t28370, t28415, t28416)
}
