//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1276/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1276(t12331: f64, t12434: f64, t10526: f64, t3537: f64, t12327: f64, t575: f64, t687: f64, t12339: f64, t23726: f64, t12346: f64, t4908: f64, t1616: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37336 = 4.0_f64 * t12331;
    let t37337 = 2.0_f64 * t12434;
    let t37339 = 4.0_f64 * t10526 * t3537;
    let t37340 = t12327 * t575;
    let t37342 = 2.0_f64 * t37340 * t687;
    let t37344 = 12.0_f64 * t23726 * t12339;
    let t37346 = 4.0_f64 * t4908 * t12346;
    let t37347 = t3537 * t3537;
    let t37349 = 4.0_f64 * t1616 * t37347;
    (t37336, t37337, t37339, t37342, t37344, t37346, t37349)
}
