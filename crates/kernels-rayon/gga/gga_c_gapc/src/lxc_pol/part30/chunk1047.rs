//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1047/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1047(t103: f64, t332: f64, t875: f64, t9739: f64, t147: f64, t19: f64, t2315: f64, t3295: f64, t966: f64, t2902: f64, t760: f64, t786: f64, t9740: f64) -> (f64, f64, f64, f64) {
    let t24086 = t9739 * t332 * t103 * t875;
    let t24092 = t3295 * t966 * t2315 * t19 * t147;
    let t24095 = t2902 * t760;
    let t24110 = t9740 * t103 * t786;
    (t24086, t24092, t24095, t24110)
}
