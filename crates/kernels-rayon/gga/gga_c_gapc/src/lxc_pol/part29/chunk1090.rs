//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1090/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1090(t11499: f64, t11938: f64, t928: f64, t1: f64, t102: f64, t8448: f64, t11813: f64, t11815: f64, t1084: f64, t33273: f64, t9415: f64, t188: f64, t20: f64, t5658: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33487 = t928 * t11499 * t11938;
    let t33490 = t8448 * t1 * t102;
    let t33491 = t11813 * t33490;
    let t33492 = t33491 * t11815;
    let t33494 = t1084 * t33273;
    let t33495 = t33494 * t9415;
    let t33498 = t20 * t5658 * t188;
    (t33487, t33490, t33491, t33492, t33494, t33495, t33498)
}
