//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1047/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1047(t3618: f64, t828: f64, t3363: f64, t5405: f64, t12287: f64, t5308: f64, t12282: f64, t5312: f64, t1260: f64, t3650: f64, t3588: f64, t73: f64) -> (f64, f64, f64, f64, f64) {
    let t12787 = t828 * t3618;
    let t12788 = t3363 * t5405;
    let t12789 = t12787 * t12788;
    let t12794 = t5308 * t12287;
    let t12797 = t5312 * t12282;
    let t12800 = t3650 * t1260;
    let t12803 = t3588 * t73;
    (t12789, t12794, t12797, t12800, t12803)
}
