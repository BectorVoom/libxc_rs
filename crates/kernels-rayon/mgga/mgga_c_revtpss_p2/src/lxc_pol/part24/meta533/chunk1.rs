//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1572/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1572(t22854: f64, t3989: f64, t221: f64, t22852: f64, t3978: f64, t9921: f64, t22956: f64, t3930: f64, t22886: f64, t9744: f64, t13790: f64, t13845: f64, t13847: f64, t73856: f64) -> (f64, f64, f64, f64, f64) {
    let t85764 = t3989 * t22854;
    let t85776 = t221 * t22852;
    let t85778 = t3978 * t9921 * t85776;
    let t85782 = t3930 * t22956;
    let t85791 = t9744 * t22886;
    let t85816 = t13845 * t13847 * t73856 * t13790;
    (t85764, t85778, t85782, t85791, t85816)
}
