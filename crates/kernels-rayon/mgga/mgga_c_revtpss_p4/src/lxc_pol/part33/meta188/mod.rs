//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk883;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk884;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk885;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta188(t342: f64, t4995: f64, t1043: f64, t3302: f64, t357: f64, t4893: f64, t1678: f64, t359: f64, t999: f64, t1089: f64, t380: f64, t4930: f64, t1024: f64, t1083: f64, t1087: f64, t1090: f64, t1093: f64, t1647: f64, t1685: f64, t1689: f64, t1692: f64, t3204: f64, t3223: f64, t3278: f64, t3287: f64, t381: f64, t4743: f64, t4857: f64, t4954: f64, t4961: f64, t4964: f64, t4967: f64, t4970: f64, t4977: f64, t4981: f64, t4984: f64, t4988: f64, t4992: f64, t989: f64, t1079: f64, t1000: f64, t1073: f64, t1076: f64, t1097: f64, t1652: f64, t1680: f64, t1696: f64, t3047: f64, t3052: f64, t3058: f64, t3063: f64, t3264: f64, t386: f64, t4747: f64, t4752: f64, t4758: f64, t4764: f64, t4773: f64, t4778: f64, t4932: f64, t4935: f64, t4941: f64, t4947: f64, t995: f64, t198: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4996, t4998, t4999, t5004, t5005, t5009, t5012) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk883(t342, t4995, t1043, t3302, t357, t4893, t1678, t359, t999, t1089, t380, t4930);
        let t5015 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk884(t1024, t1083, t1087, t1090, t1093, t1647, t1685, t1689, t1692, t3204, t3223, t3278, t3287, t342, t381, t4743, t4857, t4954, t4961, t4964, t4967, t4970, t4977, t4981, t4984, t4988, t4992, t4996, t4999, t5005, t5009, t5012, t989);
        let (t5016, t5019) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk885(t1079, t5015, t1000, t1073, t1076, t1097, t1647, t1652, t1680, t1696, t3047, t3052, t3058, t3063, t3264, t342, t386, t4743, t4747, t4752, t4758, t4764, t4773, t4778, t4932, t4935, t4941, t4947, t989, t995);
        let t5023 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk886(t198, t336);
    (t4996, t4998, t4999, t5004, t5005, t5009, t5012, t5015, t5016, t5019, t5023)
}
