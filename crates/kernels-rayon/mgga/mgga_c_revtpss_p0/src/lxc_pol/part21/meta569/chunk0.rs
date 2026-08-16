//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2267/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2267(t372: f64, t5302: f64, t4181: f64, t5405: f64, t13396: f64, t1042: f64, t3588: f64, t3603: f64, t5332: f64, t3720: f64, t15904: f64, t3623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17694 = t372 * t5302;
    let t17695 = t4181 * t5405;
    let t17696 = t17694 * t17695;
    let t17699 = t5302 * t13396;
    let t17700 = t1042 * t17699;
    let t17703 = t3603 * t3588;
    let t17704 = t5332 * t17703;
    let t17705 = t3720 * t17704;
    let t17708 = t3623 * t15904;
    (t17694, t17695, t17696, t17699, t17700, t17703, t17704, t17705, t17708)
}
