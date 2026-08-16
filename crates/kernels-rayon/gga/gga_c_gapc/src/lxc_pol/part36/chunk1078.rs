//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1078/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1078(t190: f64, t8785: f64, t1: f64, t277: f64, t11831: f64, t11752: f64, t9703: f64, t19120: f64, t3765: f64, t11730: f64, t7553: f64, t4043: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33256 = t190 * t8785;
    let t33257 = t33256 * t1;
    let t33258 = t277 * t33257;
    let t33259 = t33258 * t11831;
    let t33261 = t11752 * t9703;
    let t33263 = t19120 * t3765;
    let t33265 = t7553 * t11730;
    let t33267 = t190 * t4043;
    (t33257, t33258, t33259, t33261, t33263, t33265, t33267)
}
