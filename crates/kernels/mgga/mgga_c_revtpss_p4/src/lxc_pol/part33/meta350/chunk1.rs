//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1367/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1367<F: Float>(t13800: F, t9736: F, t241: F, t820: F, t9991: F, t5697: F, t9962: F, t5701: F, t5608: F, t5675: F, t9934: F, t2661: F) -> (F, F, F, F, F, F) {
    let t13801 = t9736 * t13800;
    let t13804 = t820 * t9991 * t241;
    let t13810 = t9962 * t5697;
    let t13813 = F::cast_from(0.20007875121765877254e-2_f64) * t9962 * t5701;
    let t13829 = t5608 * t5675;
    let t13830 = t9934 * t13829;
    let t13832 = F::cast_from(0.28582678745379824648e-4_f64) * t2661 * t13830;
    (t13801, t13804, t13810, t13813, t13829, t13832)
}
