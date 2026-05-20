//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta427 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1628;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1629;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1630;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1631;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1632;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1633;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta427<F: Float>(t1678: F, t3151: F, t3304: F, t3302: F, t4893: F, t15609: F, t15604: F, t1089: F, t1668: F, t3259: F, t15780: F, t4983: F, t3075: F, t5004: F, t359: F, t4930: F, t999: F, t1043: F, t4757: F, t3291: F, t4772: F, t3133: F, t15957: F, t4976: F, t1024: F, t1087: F, t11782: F, t11788: F, t12122: F, t12127: F, t12149: F, t1685: F, t1692: F, t3043: F, t3223: F, t3278: F, t3287: F, t3299: F, t3313: F, t4954: F, t4961: F, t4981: F, t4988: F, t5005: F, t1082: F, t15648: F, t3059: F, t3318: F, t15717: F, t3286: F, t4746: F, t1071: F, t3316: F, t342: F, t1647: F, t3298: F, t16183: F, t378: F, t4980: F, t989: F, t4995: F, t1093: F, t11940: F, t12146: F, t15670: F, t15886: F, t3204: F, t3283: F, t3288: F, t3305: F, t3317: F, t381: F, t4743: F, t4967: F, t4977: F, t4984: F, t4999: F, t16237: F, t380: F, t4998: F, t15893: F, t1086: F, t994: F, t12166: F) -> (F, F, F, F, F, F, F, F) {
        let (t16426, t16427, t16433, t16436, t16440, t16443) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1628::<F>(t1678, t3151, t3304, t3302, t4893, t15609, t15604, t1089, t1668, t3259, t15780, t4983);
        let (t16446, t16450, t16458, t16461, t16465) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1629::<F>(t3075, t5004, t359, t4930, t999, t1043, t1089, t4757, t3291, t4772, t1678, t3133);
        let t16475 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1630::<F>(t15957, t4976, t1024, t1087, t11782, t11788, t12122, t12127, t12149, t16427, t16433, t16436, t16440, t16443, t16446, t16450, t16458, t16461, t16465, t1685, t1692, t3043, t3223, t3278, t3287, t3299, t3313, t4954, t4961, t4981, t4988, t5005);
        let (t16479, t16482, t16485, t16488, t16496, t16499) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1631::<F>(t1082, t15648, t3291, t4757, t3059, t5004, t16426, t3318, t1043, t1089, t4930, t15717);
        let (t16502, t16506, t16509, t16515, t16520) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1632::<F>(t3286, t4746, t1071, t3316, t342, t1647, t3298, t1089, t16183, t378, t4980, t989);
        let t16526 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1633::<F>(t4995, t989, t1024, t1087, t1093, t11940, t12146, t15670, t15886, t16479, t16482, t16485, t16488, t16496, t16499, t16502, t16506, t16509, t16515, t16520, t3204, t3223, t3283, t3288, t3305, t3317, t381, t4743, t4967, t4977, t4984, t4999);
        let (t16529, t16534, t16537, t16540, t16544, t16551) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1634::<F>(t16237, t380, t15780, t4998, t15893, t3304, t3318, t1086, t1678, t994, t12166, t378);
    (t16475, t16526, t16529, t16534, t16537, t16540, t16544, t16551)
}
