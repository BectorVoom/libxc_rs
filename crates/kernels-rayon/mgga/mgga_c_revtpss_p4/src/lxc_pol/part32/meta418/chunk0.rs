//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1455/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1455(t19501: f64, t3095: f64, t3092: f64, t1043: f64, t3155: f64, t6271: f64, t3117: f64, t12131: f64, t357: f64, t4786: f64, t6100: f64, t1065: f64, t6244: f64) -> (f64, f64, f64, f64, f64) {
    let t19625 = t19501 * t3095;
    let t19626 = t3092 * t19625;
    let t19634 = t3155 * t1043;
    let t19635 = t6271 * t19634;
    let t19636 = t3117 * t19635;
    let t19639 = t12131 * t357;
    let t19640 = t6271 * t19639;
    let t19641 = t3117 * t19640;
    let t19644 = t6100 * t4786;
    let t19645 = t3092 * t19644;
    let t19649 = t1065 * t6244;
    (t19626, t19636, t19641, t19645, t19649)
}
