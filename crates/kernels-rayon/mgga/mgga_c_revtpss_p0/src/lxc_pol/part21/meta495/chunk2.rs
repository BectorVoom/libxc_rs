//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2090/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2090(t15655: f64, t366: f64, t3224: f64, t4845: f64, t127: f64, t371: f64, t4852: f64, t1025: f64, t1646: f64, t3056: f64) -> (f64, f64, f64, f64, f64) {
    let t15656 = t15655 * t366;
    let t15662 = 0.28582678745379824648e-3_f64 * t3224 * t4845;
    let t15666 = t371 * t127 * t4852;
    let t15668 = 0.28582678745379824648e-3_f64 * t1025 * t15666;
    let t15669 = t1646 * t3056;
    (t15656, t15662, t15666, t15668, t15669)
}
