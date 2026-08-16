//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1182/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1182(t10510: f64, t26497: f64, t10073: f64, t25402: f64, t7056: f64, t7398: f64, t26481: f64, t93182: f64, t25411: f64, t2754: f64, t676: f64, t136: f64, t2457: f64, t7423: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95779 = t26497 * t10510;
    let t95783 = t10073 * t7056 * t25402 * t7398;
    let t95785 = t26481 * t93182;
    let t95786 = t25411 * t95785;
    let t95789 = t26481 * t676 * t2754;
    let t95790 = t25411 * t95789;
    let t95793 = t7423 * t136 * t2457;
    (t95779, t95783, t95785, t95786, t95789, t95790, t95793)
}
