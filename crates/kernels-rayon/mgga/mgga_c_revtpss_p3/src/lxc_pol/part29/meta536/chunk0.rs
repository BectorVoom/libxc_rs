//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1868/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1868(t92997: f64, t92999: f64, t93007: f64, t93012: f64, t93020: f64, t26482: f64, t93321: f64, t25375: f64, t95628: f64, t136: f64, t137: f64, t2061: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t95674 = 0.15117061203111996147e0_f64 * t92997;
    let t95675 = 0.51384669507166276316e-2_f64 * t92999;
    let t95678 = 0.80328230880474379779e-6_f64 * t93007;
    let t95680 = 0.45178982497454656792e-6_f64 * t93012;
    let t95684 = 0.28900264064772933812e-2_f64 * t93020;
    let t95720 = t93321 * t26482;
    let t95722 = t25375 * t95628;
    let t95725 = t2061 * t136 * t137;
    (t95674, t95675, t95678, t95680, t95684, t95720, t95722, t95725)
}
