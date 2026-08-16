//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2098;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2099;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2100;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2101;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2102;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta606<F: Float>(t26072: F, t27888: F, t27873: F, t94886: F, t27845: F, t689: F, t25904: F, t25899: F, t94649: F, t97685: F, t25898: F, t7925: F, t94849: F, t1032: F, t5710: F, t1426: F, t7063: F, t7286: F, t27852: F, t27909: F, t4078: F, t94729: F, t94733: F, t94735: F, t94749: F, t94756: F, t94758: F, t25950: F, t25953: F, t27884: F, t13739: F, t13743: F, t25921: F, t27896: F, t28012: F, t7279: F, t7292: F, t7926: F, t94610: F, t94761: F, t94766: F, t94769: F, t94772: F, t94774: F, t94777: F, t13730: F, t2023: F, t2782: F, t10073: F, t25938: F, t27836: F, t14079: F, t26054: F, t7289: F, t97925: F, t1882: F, t25930: F, t25931: F, t25933: F, t26036: F, t27853: F, t27972: F, t7917: F, t94716: F, t94779: F, t94784: F, t94799: F, t94803: F, t94807: F, t94811: F, t94813: F, t2470: F, t27872: F, t25895: F, t1892: F, t25877: F, t25881: F, t1955: F, t14066: F, t213: F, t27960: F, t1398: F, t1445: F, t2030: F, t25909: F, t26084: F, t27868: F, t27980: F, t48025: F, t543: F, t5658: F, t5728: F, t7274: F, t7295: F, t7301: F, t7304: F, t7930: F, t94820: F, t94842: F, t94844: F, t94851: F) -> (F, F, F, F, F, F, F, F) {
        let (t97943, t97945, t97949, t97951, t97953, t97956) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2098::<F>(t26072, t27888, t27873, t94886, t27845, t689, t25904, t25899, t94649, t97685, t25898, t7925, t94849);
        let (t97960, t97961, t97966, t97969) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2099::<F>(t1032, t5710, t1426, t7063, t7286, t27852, t689, t25904, t27909, t4078, t94729, t94733, t94735, t94749, t94756, t94758, t97943, t97945, t97949, t97951, t97953, t97956);
        let t97994 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2100::<F>(t25899, t97966, t25950, t27888, t25953, t27884, t13739, t13743, t25921, t27896, t28012, t7279, t7292, t7926, t94610, t94761, t94766, t94769, t94772, t94774, t94777);
        let t98022 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2101::<F>(t13730, t2023, t2782, t10073, t25938, t27836, t14079, t26054, t7289, t97925, t1882, t25921, t25930, t25931, t25933, t26036, t27853, t27972, t7917, t94716, t94779, t94784, t94799, t94803, t94807, t94811, t94813);
        let (t98028, t98029, t98040, t98043, t98050, t98053, t98056) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2102::<F>(t2470, t27872, t25895, t1892, t7063, t25877, t25881, t1955, t97960, t14066, t213, t27960);
        let t98061 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2103::<F>(t1398, t1445, t2030, t25909, t26084, t27868, t27960, t27980, t48025, t543, t5658, t5728, t7274, t7295, t7301, t7304, t7930, t94820, t94842, t94844, t94851, t98029, t98043, t98050, t98053, t98056);
    (t97961, t97969, t97994, t98022, t98028, t98040, t98050, t98061)
}
