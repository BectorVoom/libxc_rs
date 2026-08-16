//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1178/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1178(t1285: f64, t17395: f64, t1032: f64, t5216: f64, t1246: f64, t12916: f64, t5353: f64, t3718: f64, t5347: f64, t1781: f64, t697: f64, t1222: f64) -> (f64, f64, f64, f64, f64) {
    let t17605 = t1285 * t17395;
    let t17608 = t5216 * t1032;
    let t17609 = t17608 * t1246;
    let t17617 = t12916 * t5353;
    let t17619 = 0.28582678745379824648e-3_f64 * t3718 * t17617;
    let t17620 = t12916 * t5347;
    let t17622 = 0.28582678745379824648e-3_f64 * t3718 * t17620;
    let t17628 = t697 * t1781;
    let t17629 = t1222 * t17628;
    (t17605, t17609, t17619, t17622, t17629)
}
