//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1026/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1026(t21625: f64, t5217: f64, t5963: f64, t1338: f64, t1463: f64, t136: f64, t4046: f64, t1907: f64, t199: f64, t203: f64, t1552: f64, t172: f64, t674: f64) -> (f64, f64, f64, f64, f64) {
    let t21842 = t5963 * t21625 * t5217;
    let t21991 = t1463 * t1338;
    let t22117 = 1.0_f64 / t4046 / t136;
    let t22118 = 1.0_f64 / t1907 / t199 * t203 * t22117;
    let t22327 = t1552 * t674 * t172;
    (t21842, t21991, t22117, t22118, t22327)
}
