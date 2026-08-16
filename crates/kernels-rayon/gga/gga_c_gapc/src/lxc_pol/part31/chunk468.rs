//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 468/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk468(t2553: f64, t875: f64, t2552: f64, t122: f64, t285: f64, t653: f64, t277: f64, t1087: f64, t5: f64, t1623: f64, t327: f64, t186: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2554 = t2553 * t875;
    let t2555 = t2552 * t2554;
    let t2558 = t285 * t122;
    let t2559 = t2558 * t653;
    let t2560 = t277 * t2559;
    let t2562 = t1087 * t5;
    let t2563 = t1623 * t327 * t2562;
    let t2566 = t285 * t186;
    (t2554, t2555, t2560, t2562, t2563, t2566)
}
