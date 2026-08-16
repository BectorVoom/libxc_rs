//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1590;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1591;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1592;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1593;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1594;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1595;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1596;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1597;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta438<F: Float>(t1089: F, t19829: F, t19836: F, t1024: F, t1087: F, t12146: F, t12149: F, t12154: F, t15670: F, t19608: F, t19612: F, t19617: F, t19856: F, t20113: F, t20119: F, t20123: F, t20128: F, t20133: F, t20136: F, t3204: F, t3278: F, t3287: F, t342: F, t381: F, t4961: F, t4999: F, t6365: F, t6379: F, t6389: F, t989: F, t19508: F, t19554: F, t19606: F, t1079: F, t20112: F, t225: F, t385: F, t1096: F, t6392: F, t3269: F, t1647: F, t1678: F, t378: F, t6235: F, t1076: F, t1097: F, t11187: F, t16340: F, t16374: F, t1652: F, t16597: F, t1696: F, t3264: F, t386: F, t4778: F, t4932: F, t4941: F, t6245: F, t6345: F, t6351: F, t19456: F, t996: F, t4746: F, t1695: F, t5015: F, t6343: F, t994: F, t19462: F, t4772: F, t6258: F, t1000: F, t1073: F, t11201: F, t16302: F, t16362: F, t1680: F, t3047: F, t3063: F, t4743: F, t4752: F, t4935: F, t4947: F, t6259: F, t995: F, t19390: F, t19434: F, t1100: F, t1102: F, t19143: F, t19145: F, t19149: F, t19152: F, t19153: F, t19252: F, t19258: F, t19315: F, t19317: F, t19320: F, t19323: F, t19326: F, t19329: F, t19333: F, t19337: F, t19470: F, t19473: F, t19475: F, t198: F, t336: F, t5019: F, t5023: F, t5024: F, t30: F, t265: F, t393: F, t18884: F, t19141: F, t1106: F, t1468: F, t1469: F, t1704: F, t18280: F, t18281: F, t18892: F, t395: F, t4186: F, t45: F, t4560: F, t5028: F, t5824: F, t5825: F, t605: F, t606: F, t6084: F, t6405: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3531: F, t6556: F, t6552: F, t3362: F, t3417: F, t141: F, t1121: F, t1145: F, t6461: F, t698: F, t6464: F, t6467: F, t6422: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t20149 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1590::<F>(t1089, t19829, t19836, t1024, t1087, t12146, t12149, t12154, t15670, t19608, t19612, t19617, t19856, t20113, t20119, t20123, t20128, t20133, t20136, t3204, t3278, t3287, t342, t381, t4961, t4999, t6365, t6379, t6389, t989);
        let (t20152, t20168, t20172, t20175) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1591::<F>(t19508, t19554, t19606, t20149, t1079, t20112, t225, t385, t1096, t6392, t3269, t1647, t1678);
        let t20187 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1592::<F>(t378, t6235, t1076, t1097, t11187, t16340, t16374, t1647, t1652, t16597, t1696, t19856, t20152, t20168, t20172, t20175, t3264, t342, t386, t4778, t4932, t4941, t6245, t6345, t6351, t989);
        let (t20188, t20191, t20195, t20204, t20211, t20214) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1593::<F>(t19456, t996, t1678, t4746, t1695, t5015, t3269, t6343, t994, t19462, t378, t4772);
        let t20228 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1594::<F>(t1079, t20214, t1096, t6258, t1000, t1073, t1076, t11201, t16302, t16362, t1652, t1680, t1696, t20188, t20191, t20195, t20204, t20211, t3047, t3063, t4743, t4752, t4935, t4947, t6235, t6259, t995);
        let t20234 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1595::<F>(t19390, t19434, t20187, t20228, t1100, t1102, t19143, t19145, t19149, t19152, t19153, t19252, t19258, t19315, t19317, t19320, t19323, t19326, t19329, t19333, t19337, t19470, t19473, t19475, t198, t336, t5019, t5023, t5024);
        let t20248 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1596::<F>(t30, t265, t393, t18884, t19141, t20234, t1106, t1468, t1469, t1704, t18280, t18281, t18892, t395, t4186, t45, t4560, t5028, t5824, t5825, t605, t606, t6084, t6405, t895, dens_threshold, rho0, zeta_threshold);
        let (t20256, t20261, t20263, t20266, t20268, t20272, t20273) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1597::<F>(t18280, t3531, t6556, t6552, t3362, t5825, t606, t3417, t141, t1121, t18281, t1145);
        let (t20274, t20276, t20278, t20280, t20283) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1598::<F>(t141, t20273, t6461, t698, t6464, t6467, t6422, t689);
    (t20248, t20256, t20261, t20263, t20266, t20268, t20272, t20274, t20276, t20278, t20280, t20283)
}
