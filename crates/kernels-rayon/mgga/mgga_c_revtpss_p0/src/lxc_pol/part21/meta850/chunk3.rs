//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3195/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3195(t3718: f64, t44546: f64, t5347: f64, t12916: f64, t17785: f64, t5331: f64, t3650: f64, t5390: f64, t12915: f64, t16775: f64, t247: f64, t5384: f64) -> (f64, f64, f64, f64) {
    let t58850 = t3718 * t44546 * t5347;
    let t58851 = 0.14291339372689912324e-3_f64 * t58850;
    let t58853 = t5331 * t12916 * t17785;
    let t58863 = t3650 * t5390;
    let t58868 = t5384 * t247 * t12915 * t16775;
    (t58851, t58853, t58863, t58868)
}
