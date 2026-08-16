//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 711/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk711(t13749: f64, t493: f64, t492: f64, t105: f64, t169: f64, t172: f64, t452: f64, t12032: f64, t921: f64, t2355: f64, t3718: f64, t1382: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13750 = t493 * t13749;
    let t13751 = t492 * t13750;
    let t13753 = 0.28455006635676149599e-1_f64 * t105 * t13751;
    let t13755 = t13749 * t169 * t172;
    let t13756 = t452 * t13755;
    let t13758 = 0.28455006635676149599e-1_f64 * t105 * t13756;
    let t13762 = t12032 * t921;
    let t13764 = t2355 * t3718;
    let t13765 = t3718 * t921;
    let t13766 = t1382 * t13765;
    (t13750, t13751, t13753, t13755, t13756, t13758, t13762, t13764, t13765, t13766)
}
