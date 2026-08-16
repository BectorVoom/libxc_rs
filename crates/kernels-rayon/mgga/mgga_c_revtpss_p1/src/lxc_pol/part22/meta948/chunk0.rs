//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3187/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3187(t12809: f64, t12916: f64, t17483: f64, t12772: f64, t17729: f64, t17731: f64, t3718: f64, t44546: f64, t5353: f64, t45833: f64, t58919: f64, t127: f64, t17693: f64, t17695: f64, t5302: f64) -> (f64, f64, f64, f64, f64) {
    let t59179 = t12809 * t12916 * t17483;
    let t59182 = t17729 * t12772 * t17731;
    let t59185 = t3718 * t44546 * t5353;
    let t59196 = t45833 * t58919;
    let t59220 = t17693 * t127 * t5302 * t17695;
    (t59179, t59182, t59185, t59196, t59220)
}
