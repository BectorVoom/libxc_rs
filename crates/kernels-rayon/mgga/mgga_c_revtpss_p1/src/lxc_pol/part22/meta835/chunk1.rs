//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2961/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2961(t47247: f64, t828: f64, t13967: f64, t9962: f64, t13941: f64, t46740: f64, t221: f64, t47273: f64, t13785: f64, t9816: f64, t13770: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48798 = t47247 * t828;
    let t48811 = t9962 * t13967;
    let t48813 = t46740 * t13941;
    let t48823 = t47273 * t221;
    let t48825 = t9816 * t48823 * t13785;
    let t48827 = t9775 * t13770;
    (t48798, t48811, t48813, t48823, t48825, t48827)
}
