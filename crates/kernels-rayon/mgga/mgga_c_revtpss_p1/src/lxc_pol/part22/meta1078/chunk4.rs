//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3864/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3864(t22169: f64, t46691: f64, t22173: f64, t9744: f64, t6856: f64, t9779: f64, t6880: f64, t22062: f64, t9775: f64, t13845: f64, t22145: f64, t48100: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74269 = t46691 * t22169;
    let t74271 = t9744 * t22173;
    let t74277 = t9779 * t6856;
    let t74279 = t9779 * t6880;
    let t74281 = t9775 * t22062;
    let t74288 = t13845 * t48100 * t22145;
    (t74269, t74271, t74277, t74279, t74281, t74288)
}
