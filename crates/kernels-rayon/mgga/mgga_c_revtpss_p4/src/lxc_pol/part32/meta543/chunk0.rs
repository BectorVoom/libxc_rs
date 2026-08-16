//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1854/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1854(t26481: f64, t93182: f64, t25411: f64, t136: f64, t2457: f64, t7423: f64, t25299: f64, t25431: f64, t26555: f64, t40270: f64, t25305: f64, t25410: f64, t7419: f64, t93240: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95785 = t26481 * t93182;
    let t95786 = t25411 * t95785;
    let t95793 = t7423 * t136 * t2457;
    let t95794 = t25299 * t95793;
    let t95796 = t25431 * t95785;
    let t95807 = 0.96373646535613327356e-3_f64 * t40270 * t26555;
    let t95808 = t25305 * t95793;
    let t95811 = t93240 * t25410 * t7419;
    (t95786, t95794, t95796, t95807, t95808, t95811)
}
