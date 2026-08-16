//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta176 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1054;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1055;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1056;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1057;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1058;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1059;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta176(t1071: f64, t1089: f64, t1668: f64, t378: f64, t4866: f64, t3316: f64, t342: f64, t1043: f64, t3302: f64, t357: f64, t4893: f64, t1678: f64, t359: f64, t999: f64, t380: f64, t4930: f64, t1024: f64, t1083: f64, t1087: f64, t1090: f64, t1093: f64, t1647: f64, t1685: f64, t1689: f64, t1692: f64, t3204: f64, t3223: f64, t3278: f64, t3287: f64, t381: f64, t4743: f64, t4857: f64, t4954: f64, t4961: f64, t4964: f64, t4967: f64, t4970: f64, t4977: f64, t4981: f64, t4984: f64, t989: f64, t1079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4988, t4992, t4995) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1054(t1071, t1089, t1668, t378, t4866, t3316);
        let t4996 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1055(t342, t4995);
        let t4998 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1056(t1043, t3302, t357);
        let t4999 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1057(t4893, t4998);
        let t5004 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1058(t1678, t359);
        let (t5005, t5009, t5012, t5015) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1059(t5004, t999, t1043, t1089, t1678, t380, t4930, t1024, t1083, t1087, t1090, t1093, t1647, t1685, t1689, t1692, t3204, t3223, t3278, t3287, t342, t381, t4743, t4857, t4954, t4961, t4964, t4967, t4970, t4977, t4981, t4984, t4988, t4992, t4996, t4999, t989);
        let t5016 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1060(t1079, t5015);
    (t4988, t4992, t4995, t4996, t4998, t4999, t5004, t5005, t5009, t5012, t5015, t5016)
}
