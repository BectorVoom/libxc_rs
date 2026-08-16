//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2084;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2085;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2086;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2087;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2088;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta610(t1358: f64, t212: f64, t27960: f64, t689: f64, t3923: f64, t7910: f64, t26050: f64, t27899: f64, t2453: f64, t27883: f64, t25946: f64, t27873: f64, t94890: f64, t136: f64, t2457: f64, t7929: f64, t25944: f64, t2470: f64, t27887: f64, t7284: f64, t1955: f64, t27836: f64, t4075: f64, t25934: f64, t27865: f64, t27869: f64, t543: f64, t7295: f64, t7301: f64, t94700: f64, t94703: f64, t94705: f64, t94714: f64, t94726: f64, t97855: f64, t26072: f64, t27888: f64, t94886: f64, t27845: f64, t25904: f64, t25899: f64, t94649: f64, t97685: f64, t25898: f64, t7925: f64, t94849: f64, t1032: f64, t5710: f64, t1426: f64, t7063: f64, t7286: f64, t27852: f64, t27909: f64, t4078: f64, t94729: f64, t94733: f64, t94735: f64, t94749: f64, t94756: f64, t94758: f64, t25950: f64, t25953: f64, t27884: f64, t13739: f64, t13743: f64, t25921: f64, t27896: f64, t28012: f64, t7279: f64, t7292: f64, t7926: f64, t94610: f64, t94761: f64, t94766: f64, t94769: f64, t94772: f64, t94774: f64, t94777: f64, t13730: f64, t2023: f64, t2782: f64, t10073: f64, t25938: f64, t14079: f64, t26054: f64, t7289: f64, t1882: f64, t25930: f64, t25931: f64, t25933: f64, t26036: f64, t27853: f64, t27972: f64, t7917: f64, t94716: f64, t94779: f64, t94784: f64, t94799: f64, t94803: f64, t94807: f64, t94811: f64, t94813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97908, t97909, t97915, t97917, t97920) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2084(t1358, t212, t27960, t689, t3923, t7910, t26050, t27899, t2453, t27883, t25946, t27873, t94890);
        let (t97922, t97925, t97938) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2085(t136, t2457, t7929, t25944, t2470, t27887, t7284, t1955, t27836, t4075, t25934, t27865, t27869, t543, t7295, t7301, t94700, t94703, t94705, t94714, t94726, t97855, t97908, t97909, t97915, t97917, t97920);
        let (t97943, t97945, t97949, t97951, t97953, t97956) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2086(t26072, t27888, t27873, t94886, t27845, t689, t25904, t25899, t94649, t97685, t25898, t7925, t94849);
        let (t97960, t97961, t97966, t97969) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2087(t1032, t5710, t1426, t7063, t7286, t27852, t689, t25904, t27909, t4078, t94729, t94733, t94735, t94749, t94756, t94758, t97943, t97945, t97949, t97951, t97953, t97956);
        let t97994 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2088(t25899, t97966, t25950, t27888, t25953, t27884, t13739, t13743, t25921, t27896, t28012, t7279, t7292, t7926, t94610, t94761, t94766, t94769, t94772, t94774, t94777);
        let t98022 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2089(t13730, t2023, t2782, t10073, t25938, t27836, t14079, t26054, t7289, t97925, t1882, t25921, t25930, t25931, t25933, t26036, t27853, t27972, t7917, t94716, t94779, t94784, t94799, t94803, t94807, t94811, t94813);
    (t97909, t97922, t97938, t97960, t97961, t97969, t97994, t98022)
}
