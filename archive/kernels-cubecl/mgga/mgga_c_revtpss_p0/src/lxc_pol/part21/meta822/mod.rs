//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta822 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3050;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3051;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3052;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3053;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3054;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3055;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3056;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta822<F: Float>(t10356: F, t16719: F, t1120: F, t128: F, t1469: F, t43766: F, t43860: F, t2435: F, t5048: F, t43776: F, t12305: F, t5053: F, t16739: F, t689: F, t16743: F, t16734: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t56165, t56167) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3050::<F>(t10356, t16719, t1120, t128);
        let (t56172, t56174) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3051::<F>(t10356, t1469, t43766, t128, t43860);
        let t56176 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3052::<F>(t2435, t5048);
        let (t56179, t56181) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3053::<F>(t10356, t1469, t43776, t12305, t128);
        let t56183 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3054::<F>(t2435, t5053);
        let (t56184, t56185) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3055::<F>(t56183, t16739, t689);
        let t56187 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3056::<F>(t16743, t689);
        let t56189 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3057::<F>(t16734, t689);
    (t56165, t56167, t56172, t56174, t56176, t56179, t56181, t56183, t56184, t56185, t56187, t56189)
}
