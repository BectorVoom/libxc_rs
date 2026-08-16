//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1119/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1119(t1038: f64, t11589: f64, t147: f64, t19509: f64, t457: f64, t137: f64, t27144: f64, t1552: f64, t3143: f64, t674: f64, t1666: f64, t3074: f64, t4: f64, t5216: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t27940 = t11589 * t1038 * t19509 * t147 * t457;
    let t28006 = t27144 * t137;
    let t28065 = pi * t1552 * t674 * t3143;
    let t28169 = t1666 * t3074 * t5216 * t4;
    (t27940, t28006, t28065, t28169)
}
