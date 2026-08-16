//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1103/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1103(t11794: f64, t7927: f64, t9554: f64, t126: f64, t671: f64, t128: f64, t314: f64, t786: f64, t3327: f64, t7451: f64, t15507: f64, t22: f64, t5: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t33653 = t11794 * t7927 * t9554;
    let t33655 = t126 * t671;
    let t33657 = t314 * t128;
    let t33658 = t33657 * t786;
    let t33660 = t7451 * t33655 * t3327 * t33658;
    let t33666 = 1.0_f64 / t22 / t15507 * pi * t5;
    (t33653, t33655, t33658, t33660, t33666)
}
