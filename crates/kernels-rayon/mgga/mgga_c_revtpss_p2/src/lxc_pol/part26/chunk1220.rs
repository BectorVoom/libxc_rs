//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1220/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1220(t25899: f64, t96431: f64, t1445: f64, t26354: f64, t689: f64, t1426: f64, t7507: f64, t786: f64, t3917: f64, t94701: f64, t96204: f64, t25878: f64, t96242: f64) -> (f64, f64, f64, f64, f64) {
    let t96567 = t25899 * t96431;
    let t96570 = t689 * t26354 * t1445;
    let t96576 = t786 * t7507 * t1426;
    let t96577 = t96576 * t3917;
    let t96584 = 0.51727911450665971904e-3_f64 * t94701 * t96204;
    let t96588 = t25878 * t96242;
    (t96567, t96570, t96577, t96584, t96588)
}
