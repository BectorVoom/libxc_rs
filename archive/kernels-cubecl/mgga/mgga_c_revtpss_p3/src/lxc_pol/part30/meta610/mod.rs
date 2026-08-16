//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta610 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2084;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2085;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2086;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2087;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2088;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta610<F: Float>(t1358: F, t212: F, t27960: F, t689: F, t3923: F, t7910: F, t26050: F, t27899: F, t2453: F, t27883: F, t25946: F, t27873: F, t94890: F, t136: F, t2457: F, t7929: F, t25944: F, t2470: F, t27887: F, t7284: F, t1955: F, t27836: F, t4075: F, t25934: F, t27865: F, t27869: F, t543: F, t7295: F, t7301: F, t94700: F, t94703: F, t94705: F, t94714: F, t94726: F, t97855: F, t26072: F, t27888: F, t94886: F, t27845: F, t25904: F, t25899: F, t94649: F, t97685: F, t25898: F, t7925: F, t94849: F, t1032: F, t5710: F, t1426: F, t7063: F, t7286: F, t27852: F, t27909: F, t4078: F, t94729: F, t94733: F, t94735: F, t94749: F, t94756: F, t94758: F, t25950: F, t25953: F, t27884: F, t13739: F, t13743: F, t25921: F, t27896: F, t28012: F, t7279: F, t7292: F, t7926: F, t94610: F, t94761: F, t94766: F, t94769: F, t94772: F, t94774: F, t94777: F, t13730: F, t2023: F, t2782: F, t10073: F, t25938: F, t14079: F, t26054: F, t7289: F, t1882: F, t25930: F, t25931: F, t25933: F, t26036: F, t27853: F, t27972: F, t7917: F, t94716: F, t94779: F, t94784: F, t94799: F, t94803: F, t94807: F, t94811: F, t94813: F) -> (F, F, F, F, F, F, F, F) {
        let (t97908, t97909, t97915, t97917, t97920) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2084::<F>(t1358, t212, t27960, t689, t3923, t7910, t26050, t27899, t2453, t27883, t25946, t27873, t94890);
        let (t97922, t97925, t97938) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2085::<F>(t136, t2457, t7929, t25944, t2470, t27887, t7284, t1955, t27836, t4075, t25934, t27865, t27869, t543, t7295, t7301, t94700, t94703, t94705, t94714, t94726, t97855, t97908, t97909, t97915, t97917, t97920);
        let (t97943, t97945, t97949, t97951, t97953, t97956) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2086::<F>(t26072, t27888, t27873, t94886, t27845, t689, t25904, t25899, t94649, t97685, t25898, t7925, t94849);
        let (t97960, t97961, t97966, t97969) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2087::<F>(t1032, t5710, t1426, t7063, t7286, t27852, t689, t25904, t27909, t4078, t94729, t94733, t94735, t94749, t94756, t94758, t97943, t97945, t97949, t97951, t97953, t97956);
        let t97994 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2088::<F>(t25899, t97966, t25950, t27888, t25953, t27884, t13739, t13743, t25921, t27896, t28012, t7279, t7292, t7926, t94610, t94761, t94766, t94769, t94772, t94774, t94777);
        let t98022 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2089::<F>(t13730, t2023, t2782, t10073, t25938, t27836, t14079, t26054, t7289, t97925, t1882, t25921, t25930, t25931, t25933, t26036, t27853, t27972, t7917, t94716, t94779, t94784, t94799, t94803, t94807, t94811, t94813);
    (t97909, t97922, t97938, t97960, t97961, t97969, t97994, t98022)
}
