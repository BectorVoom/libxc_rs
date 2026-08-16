//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk883;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk884;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk885;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta188<F: Float>(t342: F, t4995: F, t1043: F, t3302: F, t357: F, t4893: F, t1678: F, t359: F, t999: F, t1089: F, t380: F, t4930: F, t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t1647: F, t1685: F, t1689: F, t1692: F, t3204: F, t3223: F, t3278: F, t3287: F, t381: F, t4743: F, t4857: F, t4954: F, t4961: F, t4964: F, t4967: F, t4970: F, t4977: F, t4981: F, t4984: F, t4988: F, t4992: F, t989: F, t1079: F, t1000: F, t1073: F, t1076: F, t1097: F, t1652: F, t1680: F, t1696: F, t3047: F, t3052: F, t3058: F, t3063: F, t3264: F, t386: F, t4747: F, t4752: F, t4758: F, t4764: F, t4773: F, t4778: F, t4932: F, t4935: F, t4941: F, t4947: F, t995: F, t198: F, t336: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4996, t4998, t4999, t5004, t5005, t5009, t5012) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk883::<F>(t342, t4995, t1043, t3302, t357, t4893, t1678, t359, t999, t1089, t380, t4930);
        let t5015 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk884::<F>(t1024, t1083, t1087, t1090, t1093, t1647, t1685, t1689, t1692, t3204, t3223, t3278, t3287, t342, t381, t4743, t4857, t4954, t4961, t4964, t4967, t4970, t4977, t4981, t4984, t4988, t4992, t4996, t4999, t5005, t5009, t5012, t989);
        let (t5016, t5019) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk885::<F>(t1079, t5015, t1000, t1073, t1076, t1097, t1647, t1652, t1680, t1696, t3047, t3052, t3058, t3063, t3264, t342, t386, t4743, t4747, t4752, t4758, t4764, t4773, t4778, t4932, t4935, t4941, t4947, t989, t995);
        let t5023 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk886::<F>(t198, t336);
    (t4996, t4998, t4999, t5004, t5005, t5009, t5012, t5015, t5016, t5019, t5023)
}
