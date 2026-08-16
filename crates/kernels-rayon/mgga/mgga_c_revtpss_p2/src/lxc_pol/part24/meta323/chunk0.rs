//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1119/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1119(t30: f64, t13611: f64, t1468: f64, t6785: f64, t22670: f64, t513: f64, t5549: f64, t5824: f64, t9335: f64, t1711: f64, t6792: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t22768 = 0.17544670867903938621e1_f64 * t13611;
    let t22769 = t6785 * t1468;
    let t22777 = piecewise3(t31, 0.0_f64, -8.0_f64 / 27.0_f64 * t9335 * t22769 + 4.0_f64 / 3.0_f64 * t5549 * t5824 + 4.0_f64 / 3.0_f64 * t513 * t22670);
    let t22778 = t6792 * t1711;
    let t22783 = -t22670;
    (t22768, t22769, t22777, t22778, t22783)
}
