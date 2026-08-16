//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1136;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1137;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1138;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1139;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1140;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1141;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta190<F: Float>(t1235: F, t5362: F, t1219: F, t1778: F, t1225: F, t4186: F, t1012: F, t1222: F, t3657: F, t3658: F, t3679: F, t3684: F, t3718: F, t5340: F, t5343: F, t5348: F, t5354: F, t5358: F, t1010: F, t1480: F, t1715: F, t3634: F, t247: F, t1261: F, t1260: F, t1785: F, t3670: F, t3719: F, t5230: F, t1802: F, t369: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5363, t5366, t5368, t5369, t5372) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1136::<F>(t1235, t5362, t1219, t1778, t1225, t4186, t1012, t1222, t3657, t3658, t3679, t3684, t3718, t5340, t5343, t5348, t5354, t5358);
        let t5373 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1137::<F>(t1010, t1480);
        let t5378 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1138::<F>(t1715, t3634, t247);
        let (t5379, t5381) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1139::<F>(t1261, t5378, t1260, t1785);
        let t5384 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1140::<F>(t1260, t3670);
        let t5386 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1141::<F>(t3719, t5230, t247);
        let t5390 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1142::<F>(t1802, t369, t475);
    (t5363, t5366, t5368, t5369, t5372, t5373, t5378, t5379, t5381, t5384, t5386, t5390)
}
