//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1154/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1154(t1084: f64, t33273: f64, t9415: f64, t188: f64, t20: f64, t5658: f64, t10293: f64, t29664: f64, t3437: f64, t11449: f64, t11815: f64, t190: f64, t2786: f64) -> (f64, f64, f64, f64, f64) {
    let t33494 = t1084 * t33273;
    let t33495 = t33494 * t9415;
    let t33498 = t20 * t5658 * t188;
    let t33501 = t3437 * t33498 * t10293 * t29664;
    let t33505 = t2786 * t190 * t11449 * t11815;
    (t33494, t33495, t33498, t33501, t33505)
}
