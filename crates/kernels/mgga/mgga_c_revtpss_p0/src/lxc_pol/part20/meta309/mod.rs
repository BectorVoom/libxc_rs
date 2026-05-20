//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1208;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1209;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1210;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta309<F: Float>(t1211: F, t12646: F, t1214: F, t3790: F, t1277: F, t3552: F, t487: F, t1208: F, t3551: F, t1210: F, t1215: F, t12600: F, t12603: F, t12607: F, t12622: F, t12628: F, t12630: F, t12633: F, t12641: F, t1295: F, t3556: F, t3567: F, t3569: F, t3572: F, t3576: F, t3585: F, t3732: F, t3791: F, t1209: F, t3727: F, t460: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t459: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12647, t12651, t12654, t12657) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1208::<F>(t1211, t12646, t1214, t3790, t1277, t3552, t487, t1208, t3551);
        let (t12658, t12663) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1209::<F>(t12657, t487, t1210, t1215, t12600, t12603, t12607, t12622, t12628, t12630, t12633, t12641, t12647, t12651, t12654, t1295, t3556, t3567, t3569, t3572, t3576, t3585, t3732, t3791);
        let (t12666, t12673, t12689) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1210::<F>(t1209, t3727, t460, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
        let t12690 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1211::<F>(t12689, t459);
    (t12647, t12651, t12654, t12657, t12658, t12663, t12666, t12673, t12689, t12690)
}
