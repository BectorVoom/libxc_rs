//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta223 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1050;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1051;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1052;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1053;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1054;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta223<F: Float>(t4893: F, t4998: F, t1678: F, t359: F, t999: F, t1043: F, t1089: F, t380: F, t4930: F, t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t1647: F, t1685: F, t1689: F, t1692: F, t3204: F, t3223: F, t3278: F, t3287: F, t342: F, t381: F, t4743: F, t4857: F, t4954: F, t4961: F, t4964: F, t4967: F, t4970: F, t4977: F, t4981: F, t4984: F, t4988: F, t4992: F, t4996: F, t989: F, t1079: F, t1000: F, t1073: F, t1076: F, t1097: F, t1652: F, t1680: F, t1696: F, t3047: F, t3052: F, t3058: F, t3063: F, t3264: F, t386: F, t4747: F, t4752: F, t4758: F, t4764: F, t4773: F, t4778: F, t4932: F, t4935: F, t4941: F, t4947: F, t995: F, t198: F, t336: F, t1699: F, t3336: F, t1100: F, t1102: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4716: F, t4718: F, t4721: F, t4723: F, t4727: F, t4731: F, t4736: F, t30: F, t265: F, t393: F, t4560: F, t1106: F, t1468: F, t1469: F, t1587: F, t1704: F, t395: F, t4186: F, t45: F, t4568: F, t605: F, t606: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4999, t5004, t5005, t5009, t5012, t5015) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1050::<F>(t4893, t4998, t1678, t359, t999, t1043, t1089, t380, t4930, t1024, t1083, t1087, t1090, t1093, t1647, t1685, t1689, t1692, t3204, t3223, t3278, t3287, t342, t381, t4743, t4857, t4954, t4961, t4964, t4967, t4970, t4977, t4981, t4984, t4988, t4992, t4996, t989);
        let t5016 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1051::<F>(t1079, t5015);
        let t5019 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1052::<F>(t1000, t1073, t1076, t1097, t1647, t1652, t1680, t1696, t3047, t3052, t3058, t3063, t3264, t342, t386, t4743, t4747, t4752, t4758, t4764, t4773, t4778, t4932, t4935, t4941, t4947, t5016, t989, t995);
        let t5023 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1053::<F>(t198, t336);
        let (t5024, t5027) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1054::<F>(t1699, t3336, t1100, t1102, t198, t336, t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736, t5019, t5023);
        let (t5028, t5035) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1055::<F>(t30, t265, t393, t4560, t5027, t1106, t1468, t1469, t1587, t1704, t395, t4186, t45, t4568, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
    (t4999, t5004, t5005, t5009, t5012, t5015, t5016, t5019, t5023, t5024, t5028, t5035)
}
