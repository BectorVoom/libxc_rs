//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1338/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1338(t1656: f64, t3259: f64, t3260: f64, t41437: f64, t520: f64, t1640: f64, t3384: f64, t19539: f64, t5736: f64, t3366: f64, t12828: f64, t3326: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65695 = t1656 * t3259;
    let t65696 = t65695 * t3260;
    let t65703 = t41437 * t520;
    let t65711 = t65695 * t520;
    let t65715 = t1640 * t3384;
    let t65719 = t5736 * t19539;
    let t65722 = t1640 * t3366;
    let t65729 = t12828 * t3326;
    (t65696, t65703, t65711, t65715, t65719, t65722, t65729)
}
