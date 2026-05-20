//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2090;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2091;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2092;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2093;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta611<F: Float>(t2470: F, t27872: F, t25895: F, t1892: F, t7063: F, t25877: F, t25881: F, t1955: F, t97960: F, t14066: F, t213: F, t27960: F, t1398: F, t1445: F, t2030: F, t25909: F, t26084: F, t27868: F, t27980: F, t48025: F, t543: F, t5658: F, t5728: F, t7274: F, t7295: F, t7301: F, t7304: F, t7930: F, t94820: F, t94842: F, t94844: F, t94851: F, t1883: F, t4077: F, t27902: F, t686: F, t72: F, t25878: F, t97732: F, t27840: F, t689: F, t94674: F, t94669: F, t26069: F, t97922: F, t28011: F, t7284: F, t7289: F, t14269: F, t25885: F, t25931: F, t27837: F, t28008: F, t7279: F, t7308: F, t94823: F, t94854: F, t94857: F, t94865: F, t94867: F, t10073: F, t25937: F, t7282: F, t7910: F, t25899: F, t97899: F, t25953: F, t27899: F, t25981: F, t5677: F, t820: F, t844: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98028, t98029, t98040, t98043, t98050, t98053, t98056) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2090::<F>(t2470, t27872, t25895, t1892, t7063, t25877, t25881, t1955, t97960, t14066, t213, t27960);
        let t98061 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2091::<F>(t1398, t1445, t2030, t25909, t26084, t27868, t27960, t27980, t48025, t543, t5658, t5728, t7274, t7295, t7301, t7304, t7930, t94820, t94842, t94844, t94851, t98029, t98043, t98050, t98053, t98056);
        let (t98062, t98067, t98069, t98071, t98078, t98081, t98084) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2092::<F>(t1883, t4077, t27902, t686, t72, t25878, t97732, t27840, t689, t94674, t94669, t26069, t97922);
        let t98092 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2093::<F>(t28011, t686, t72, t7284, t7289, t14269, t25885, t25931, t27837, t28008, t7279, t7308, t94823, t94854, t94857, t94865, t94867, t98062, t98069, t98071, t98078, t98081, t98084);
        let (t98099, t98101, t98104, t98108) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2094::<F>(t10073, t25937, t7282, t7910, t25899, t97899, t25953, t27899, t25981, t5677, t820, t844);
    (t98028, t98040, t98050, t98061, t98067, t98092, t98099, t98101, t98104, t98108)
}
