//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1223/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1223(t26594: f64, t92232: f64, t26576: f64, t37013: f64, t7579: f64, t26602: f64, t7580: f64, t92226: f64, t26579: f64, t7583: f64, t9229: f64, t26580: f64, t26611: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92307 = t26594 * t92232;
    let t92310 = t37013 * t7579 * t26576;
    let t92312 = t26602 * t26576;
    let t92314 = t7580 * t92226;
    let t92316 = t7580 * t92232;
    let t92319 = t9229 * t26579 * t7583;
    let t92321 = t26580 * t26611;
    (t92307, t92310, t92312, t92314, t92316, t92319, t92321)
}
