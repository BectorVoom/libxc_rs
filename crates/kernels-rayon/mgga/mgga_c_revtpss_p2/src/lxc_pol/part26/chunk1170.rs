//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1170/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1170(t2470: f64, t26543: f64, t7058: f64, t122: f64, t25412: f64, t72: f64, t7398: f64, t25431: f64, t2646: f64, t26481: f64, t676: f64, t26482: f64, t93374: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95575 = t26543 * t2470;
    let t95576 = t7058 * t95575;
    let t95593 = t7398 * t72 * t122 * t25412;
    let t95594 = t25431 * t95593;
    let t95597 = t26481 * t676 * t2646;
    let t95598 = t25431 * t95597;
    let t95604 = t93374 * t26482;
    (t95575, t95576, t95593, t95594, t95597, t95598, t95604)
}
