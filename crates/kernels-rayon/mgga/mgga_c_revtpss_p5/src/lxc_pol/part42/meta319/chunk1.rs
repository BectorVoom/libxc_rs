//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1096/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1096(t13846: f64, t220: f64, t124: f64, t1882: f64, t5675: f64, t13845: f64, t5609: f64, t9794: f64, t9793: f64, t221: f64, t5627: f64, t9921: f64) -> (f64, f64, f64, f64, f64) {
    let t13847 = t13846 * t220;
    let t13848 = t124 * t1882;
    let t13850 = t13847 * t13848 * t5675;
    let t13851 = t13845 * t13850;
    let t13857 = t9794 * t5609;
    let t13858 = t9793 * t13857;
    let t13877 = t221 * t5627;
    let t13878 = t9921 * t13877;
    (t13847, t13848, t13851, t13858, t13878)
}
