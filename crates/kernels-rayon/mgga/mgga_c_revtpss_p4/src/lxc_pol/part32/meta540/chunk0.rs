//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1851/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1851(t25387: f64, t95628: f64, t11015: f64, t7388: f64, t92975: f64, t92988: f64, t92995: f64, t92997: f64, t92999: f64, t93007: f64, t93012: f64, t93020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t95629 = t25387 * t95628;
    let t95632 = 0.30356481678079769392e-1_f64 * t7388 * t11015;
    let t95666 = 0.18295201011342718161e-3_f64 * t92975;
    let t95671 = 0.3252886739816735289e-3_f64 * t92988;
    let t95673 = 455.0_f64 / 648.0_f64 * t92995;
    let t95674 = 0.15117061203111996147e0_f64 * t92997;
    let t95675 = 0.51384669507166276316e-2_f64 * t92999;
    let t95678 = 0.80328230880474379779e-6_f64 * t93007;
    let t95680 = 0.45178982497454656792e-6_f64 * t93012;
    let t95684 = 0.28900264064772933812e-2_f64 * t93020;
    (t95629, t95632, t95666, t95671, t95673, t95674, t95675, t95678, t95680, t95684)
}
