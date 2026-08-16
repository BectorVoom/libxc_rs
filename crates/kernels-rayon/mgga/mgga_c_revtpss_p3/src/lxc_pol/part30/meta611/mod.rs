//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2090;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2091;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2092;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2093;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta611(t2470: f64, t27872: f64, t25895: f64, t1892: f64, t7063: f64, t25877: f64, t25881: f64, t1955: f64, t97960: f64, t14066: f64, t213: f64, t27960: f64, t1398: f64, t1445: f64, t2030: f64, t25909: f64, t26084: f64, t27868: f64, t27980: f64, t48025: f64, t543: f64, t5658: f64, t5728: f64, t7274: f64, t7295: f64, t7301: f64, t7304: f64, t7930: f64, t94820: f64, t94842: f64, t94844: f64, t94851: f64, t1883: f64, t4077: f64, t27902: f64, t686: f64, t72: f64, t25878: f64, t97732: f64, t27840: f64, t689: f64, t94674: f64, t94669: f64, t26069: f64, t97922: f64, t28011: f64, t7284: f64, t7289: f64, t14269: f64, t25885: f64, t25931: f64, t27837: f64, t28008: f64, t7279: f64, t7308: f64, t94823: f64, t94854: f64, t94857: f64, t94865: f64, t94867: f64, t10073: f64, t25937: f64, t7282: f64, t7910: f64, t25899: f64, t97899: f64, t25953: f64, t27899: f64, t25981: f64, t5677: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98028, t98029, t98040, t98043, t98050, t98053, t98056) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2090(t2470, t27872, t25895, t1892, t7063, t25877, t25881, t1955, t97960, t14066, t213, t27960);
        let t98061 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2091(t1398, t1445, t2030, t25909, t26084, t27868, t27960, t27980, t48025, t543, t5658, t5728, t7274, t7295, t7301, t7304, t7930, t94820, t94842, t94844, t94851, t98029, t98043, t98050, t98053, t98056);
        let (t98062, t98067, t98069, t98071, t98078, t98081, t98084) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2092(t1883, t4077, t27902, t686, t72, t25878, t97732, t27840, t689, t94674, t94669, t26069, t97922);
        let t98092 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2093(t28011, t686, t72, t7284, t7289, t14269, t25885, t25931, t27837, t28008, t7279, t7308, t94823, t94854, t94857, t94865, t94867, t98062, t98069, t98071, t98078, t98081, t98084);
        let (t98099, t98101, t98104, t98108) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2094(t10073, t25937, t7282, t7910, t25899, t97899, t25953, t27899, t25981, t5677, t820, t844);
    (t98028, t98040, t98050, t98061, t98067, t98092, t98099, t98101, t98104, t98108)
}
