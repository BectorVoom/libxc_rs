//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta208 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1257;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1258;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1259;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1260;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1261;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1262;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta208<F: Float>(t1071: F, t1089: F, t1668: F, t378: F, t4866: F, t3316: F, t342: F, t1043: F, t3302: F, t357: F, t4893: F, t1678: F, t359: F, t999: F, t380: F, t4930: F, t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t1647: F, t1685: F, t1689: F, t1692: F, t3204: F, t3223: F, t3278: F, t3287: F, t381: F, t4743: F, t4857: F, t4954: F, t4961: F, t4964: F, t4967: F, t4970: F, t4977: F, t4981: F, t4984: F, t989: F, t1079: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4988, t4992, t4995) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1257::<F>(t1071, t1089, t1668, t378, t4866, t3316);
        let t4996 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1258::<F>(t342, t4995);
        let t4998 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1259::<F>(t1043, t3302, t357);
        let t4999 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1260::<F>(t4893, t4998);
        let t5004 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1261::<F>(t1678, t359);
        let (t5005, t5009, t5012, t5015) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1262::<F>(t5004, t999, t1043, t1089, t1678, t380, t4930, t1024, t1083, t1087, t1090, t1093, t1647, t1685, t1689, t1692, t3204, t3223, t3278, t3287, t342, t381, t4743, t4857, t4954, t4961, t4964, t4967, t4970, t4977, t4981, t4984, t4988, t4992, t4996, t4999, t989);
        let t5016 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1263::<F>(t1079, t5015);
    (t4988, t4992, t4995, t4996, t4998, t4999, t5004, t5005, t5009, t5012, t5015, t5016)
}
