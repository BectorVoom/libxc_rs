//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1817/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1817(t73515: f64, t74106: f64, t48280: f64, t48282: f64, t48285: f64, t48287: f64, t48290: f64, t47067: f64, t47070: f64, t47072: f64, t47074: f64, t47076: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t91974 = 0.14649157844805236043e-2_f64 * t73515;
    let t91975 = 0.10389515463408878255e3_f64 * t74106;
    let t91976 = 0.22787578869697033845e-2_f64 * t48280;
    let t91977 = 0.14035736694323150897e2_f64 * t48282;
    let t91978 = 0.14035736694323150897e2_f64 * t48285;
    let t91979 = 96.0_f64 * t48287;
    let t91980 = 576.0_f64 * t48290;
    let t91981 = t91974 + t47067 - t91975 + t47070 - t47072 - t47074 - t91976 - t91977 - t47076 + t91978 - t91979 - t91980;
    (t91974, t91975, t91976, t91977, t91978, t91979, t91980, t91981)
}
