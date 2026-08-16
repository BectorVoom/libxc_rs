//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1713/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1713(t12916: f64, t5353: f64, t3718: f64, t5347: f64, t3568: f64, t471: f64, t5351: f64, t3720: f64, t1781: f64, t697: f64, t1222: f64, t5284: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17617 = t12916 * t5353;
    let t17619 = 0.28582678745379824648e-3_f64 * t3718 * t17617;
    let t17620 = t12916 * t5347;
    let t17622 = 0.28582678745379824648e-3_f64 * t3718 * t17620;
    let t17623 = t471 * t3568;
    let t17624 = t5351 * t17623;
    let t17625 = t3720 * t17624;
    let t17628 = t697 * t1781;
    let t17629 = t1222 * t17628;
    let t17633 = t5284 * t73;
    (t17617, t17619, t17620, t17622, t17625, t17628, t17629, t17633)
}
