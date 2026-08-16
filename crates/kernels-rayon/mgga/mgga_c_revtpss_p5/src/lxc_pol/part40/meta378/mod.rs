//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1351;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1352;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1353;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1354;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1355;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1356;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta378(t1678: f64, t3151: f64, t3304: f64, t3302: f64, t4893: f64, t15609: f64, t15604: f64, t1089: f64, t1668: f64, t3259: f64, t15780: f64, t4983: f64, t3075: f64, t5004: f64, t359: f64, t4930: f64, t999: f64, t1043: f64, t4757: f64, t3291: f64, t4772: f64, t3133: f64, t15957: f64, t4976: f64, t1024: f64, t1087: f64, t11782: f64, t11788: f64, t12122: f64, t12127: f64, t12149: f64, t1685: f64, t1692: f64, t3043: f64, t3223: f64, t3278: f64, t3287: f64, t3299: f64, t3313: f64, t4954: f64, t4961: f64, t4981: f64, t4988: f64, t5005: f64, t1082: f64, t15648: f64, t3059: f64, t3318: f64, t15717: f64, t3286: f64, t4746: f64, t1071: f64, t3316: f64, t342: f64, t1647: f64, t3298: f64, t16183: f64, t378: f64, t4980: f64, t989: f64, t4995: f64, t1093: f64, t11940: f64, t12146: f64, t15670: f64, t15886: f64, t3204: f64, t3283: f64, t3288: f64, t3305: f64, t3317: f64, t381: f64, t4743: f64, t4967: f64, t4977: f64, t4984: f64, t4999: f64, t16237: f64, t380: f64, t4998: f64, t15893: f64, t1086: f64, t994: f64, t12166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16426, t16427, t16433, t16436, t16440, t16443) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1351(t1678, t3151, t3304, t3302, t4893, t15609, t15604, t1089, t1668, t3259, t15780, t4983);
        let (t16446, t16450, t16458, t16461, t16465) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1352(t3075, t5004, t359, t4930, t999, t1043, t1089, t4757, t3291, t4772, t1678, t3133);
        let t16475 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1353(t15957, t4976, t1024, t1087, t11782, t11788, t12122, t12127, t12149, t16427, t16433, t16436, t16440, t16443, t16446, t16450, t16458, t16461, t16465, t1685, t1692, t3043, t3223, t3278, t3287, t3299, t3313, t4954, t4961, t4981, t4988, t5005);
        let (t16479, t16482, t16485, t16488, t16496, t16499) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1354(t1082, t15648, t3291, t4757, t3059, t5004, t16426, t3318, t1043, t1089, t4930, t15717);
        let (t16502, t16506, t16509, t16515, t16520) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1355(t3286, t4746, t1071, t3316, t342, t1647, t3298, t1089, t16183, t378, t4980, t989);
        let t16526 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1356(t4995, t989, t1024, t1087, t1093, t11940, t12146, t15670, t15886, t16479, t16482, t16485, t16488, t16496, t16499, t16502, t16506, t16509, t16515, t16520, t3204, t3223, t3283, t3288, t3305, t3317, t381, t4743, t4967, t4977, t4984, t4999);
        let (t16529, t16534, t16537, t16540, t16544, t16551) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1357(t16237, t380, t15780, t4998, t15893, t3304, t3318, t1086, t1678, t994, t12166, t378);
    (t16475, t16526, t16529, t16534, t16537, t16540, t16544, t16551)
}
