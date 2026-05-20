//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta721 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2482;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta721<F: Float>(t2482: F, t814: F, t9991: F, t46917: F, t5706: F, t241: F, t47201: F, t820: F, t47198: F, t5665: F, t5629: F, t9779: F, t5661: F, t9909: F, t47247: F, t828: F, t13941: F, t46740: F, t221: F, t47273: F, t13770: F, t9775: F, t40690: F, t5610: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48731, t48756, t48759, t48792, t48794) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2482::<F>(t2482, t814, t9991, t46917, t5706, t241, t47201, t820, t47198, t5665, t5629, t9779);
        let (t48797, t48798, t48814, t48823, t48827, t48829) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2483::<F>(t5661, t9909, t47247, t828, t13941, t46740, t221, t47273, t13770, t9775, t40690, t5610);
    (t48731, t48756, t48759, t48792, t48794, t48797, t48798, t48814, t48823, t48827, t48829)
}
