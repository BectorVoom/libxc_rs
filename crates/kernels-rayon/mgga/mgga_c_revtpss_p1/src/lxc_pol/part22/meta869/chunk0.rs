//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3027/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3027(t14923: f64, t14927: f64, t10811: f64, t14697: f64, t40672: f64, t828: f64, t10905: f64, t14825: f64, t14829: f64, t14819: f64, t40517: f64, t14910: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51000 = t14923 * t14927;
    let t51006 = t10811 * t14697;
    let t51014 = t40672 * t828;
    let t51026 = t10905 * t14825;
    let t51028 = t10905 * t14829;
    let t51042 = t40517 * t14819;
    let t51047 = t10811 * t14910;
    (t51000, t51006, t51014, t51026, t51028, t51042, t51047)
}
