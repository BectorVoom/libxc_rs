//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1357/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1357(t12046: f64, t15905: f64, t994: f64, t1014: f64, t11150: f64, t221: f64, t345: f64, t346: f64, t624: f64, t1065: f64, t215: f64, t373: f64, t675: f64) -> (f64, f64, f64, f64, f64) {
    let t42690 = t994 * t12046 * t15905;
    let t42731 = t1014 * t11150;
    let t42745 = 5.0_f64 / 486.0_f64 * t345 * t221 * t624 * t346;
    let t42778 = t215 * t1065;
    let t42792 = t675 * t373;
    (t42690, t42731, t42745, t42778, t42792)
}
