//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3204/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3204(t17500: f64, t372: f64, t13142: f64, t56878: f64, t12866: f64, t17514: f64, t56756: f64, t12916: f64, t17723: f64, t3718: f64, t13043: f64, t1774: f64) -> (f64, f64, f64, f64, f64) {
    let t59062 = t372 * t17500;
    let t59066 = t13142 * t56878;
    let t59078 = t12866 * t56756 * t17514;
    let t59094 = t3718 * t12916 * t17723;
    let t59096 = t1774 * t13043;
    (t59062, t59066, t59078, t59094, t59096)
}
