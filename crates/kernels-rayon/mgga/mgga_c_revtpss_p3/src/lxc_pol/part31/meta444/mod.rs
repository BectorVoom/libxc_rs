//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1582;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1583;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1584;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1585;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1586;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1587;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1588;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta444(t20112: f64, t380: f64, t1043: f64, t1089: f64, t6343: f64, t1668: f64, t4930: f64, t16449: f64, t1651: f64, t4772: f64, t5004: f64, t20089: f64, t19829: f64, t19836: f64, t1024: f64, t1087: f64, t12146: f64, t12149: f64, t12154: f64, t15670: f64, t19608: f64, t19612: f64, t19617: f64, t19856: f64, t3204: f64, t3278: f64, t3287: f64, t342: f64, t381: f64, t4961: f64, t4999: f64, t6365: f64, t6379: f64, t6389: f64, t989: f64, t19508: f64, t19554: f64, t19606: f64, t1079: f64, t225: f64, t385: f64, t1096: f64, t6392: f64, t3269: f64, t1647: f64, t1678: f64, t378: f64, t6235: f64, t1076: f64, t1097: f64, t11187: f64, t16340: f64, t16374: f64, t1652: f64, t16597: f64, t1696: f64, t3264: f64, t386: f64, t4778: f64, t4932: f64, t4941: f64, t6245: f64, t6345: f64, t6351: f64, t19456: f64, t996: f64, t4746: f64, t1695: f64, t5015: f64, t994: f64, t19462: f64, t6258: f64, t1000: f64, t1073: f64, t11201: f64, t16302: f64, t16362: f64, t1680: f64, t3047: f64, t3063: f64, t4743: f64, t4752: f64, t4935: f64, t4947: f64, t6259: f64, t995: f64, t19390: f64, t19434: f64, t1100: f64, t1102: f64, t19143: f64, t19145: f64, t19149: f64, t19152: f64, t19153: f64, t19252: f64, t19258: f64, t19315: f64, t19317: f64, t19320: f64, t19323: f64, t19326: f64, t19329: f64, t19333: f64, t19337: f64, t19470: f64, t19473: f64, t19475: f64, t198: f64, t336: f64, t5019: f64, t5023: f64, t5024: f64, t30: f64, t265: f64, t393: f64, t18884: f64, t19141: f64, t1106: f64, t1468: f64, t1469: f64, t1704: f64, t18280: f64, t18281: f64, t18892: f64, t395: f64, t4186: f64, t45: f64, t4560: f64, t5028: f64, t5824: f64, t5825: f64, t605: f64, t606: f64, t6084: f64, t6405: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20113, t20119, t20123, t20128, t20133, t20136) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1582(t20112, t380, t1043, t1089, t6343, t1668, t4930, t16449, t1651, t4772, t5004, t20089);
        let t20149 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1583(t1089, t19829, t19836, t1024, t1087, t12146, t12149, t12154, t15670, t19608, t19612, t19617, t19856, t20113, t20119, t20123, t20128, t20133, t20136, t3204, t3278, t3287, t342, t381, t4961, t4999, t6365, t6379, t6389, t989);
        let (t20151, t20152, t20168, t20172, t20175) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1584(t19508, t19554, t19606, t20149, t1079, t20112, t225, t385, t1096, t6392, t3269, t1647, t1678);
        let t20187 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1585(t378, t6235, t1076, t1097, t11187, t16340, t16374, t1647, t1652, t16597, t1696, t19856, t20152, t20168, t20172, t20175, t3264, t342, t386, t4778, t4932, t4941, t6245, t6345, t6351, t989);
        let (t20188, t20191, t20195, t20204, t20211, t20214) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1586(t19456, t996, t1678, t4746, t1695, t5015, t3269, t6343, t994, t19462, t378, t4772);
        let (t20215, t20219, t20228) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1587(t1079, t20214, t1096, t6258, t1000, t1073, t1076, t11201, t16302, t16362, t1652, t1680, t1696, t20188, t20191, t20195, t20204, t20211, t3047, t3063, t4743, t4752, t4935, t4947, t6235, t6259, t995);
        let (t20230, t20234) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1588(t19390, t19434, t20187, t20228, t1100, t1102, t19143, t19145, t19149, t19152, t19153, t19252, t19258, t19315, t19317, t19320, t19323, t19326, t19329, t19333, t19337, t19470, t19473, t19475, t198, t336, t5019, t5023, t5024);
        let t20248 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1589(t30, t265, t393, t18884, t19141, t20234, t1106, t1468, t1469, t1704, t18280, t18281, t18892, t395, t4186, t45, t4560, t5028, t5824, t5825, t605, t606, t6084, t6405, t895, dens_threshold, rho0, zeta_threshold);
    (t20151, t20152, t20172, t20188, t20195, t20215, t20219, t20230, t20248)
}
