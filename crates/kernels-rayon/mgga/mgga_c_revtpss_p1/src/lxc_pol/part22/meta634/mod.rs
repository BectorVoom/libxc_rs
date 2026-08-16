//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2557;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2558;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2559;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2560;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta634(t19456: f64, t996: f64, t1678: f64, t4746: f64, t1695: f64, t5015: f64, t3269: f64, t6343: f64, t994: f64, t19462: f64, t378: f64, t4772: f64, t1079: f64, t1096: f64, t6258: f64, t1000: f64, t1073: f64, t1076: f64, t11201: f64, t16302: f64, t16362: f64, t1652: f64, t1680: f64, t1696: f64, t3047: f64, t3063: f64, t4743: f64, t4752: f64, t4935: f64, t4947: f64, t6235: f64, t6259: f64, t995: f64, t19390: f64, t19434: f64, t20187: f64, t1100: f64, t1102: f64, t19143: f64, t19145: f64, t19149: f64, t19152: f64, t19153: f64, t19252: f64, t19258: f64, t19315: f64, t19317: f64, t19320: f64, t19323: f64, t19326: f64, t19329: f64, t19333: f64, t19337: f64, t19470: f64, t19473: f64, t19475: f64, t198: f64, t336: f64, t5019: f64, t5023: f64, t5024: f64, t30: f64, t265: f64, t393: f64, t18884: f64, t19141: f64, t1106: f64, t1468: f64, t1469: f64, t1704: f64, t18280: f64, t18281: f64, t18892: f64, t395: f64, t4186: f64, t45: f64, t4560: f64, t5028: f64, t5824: f64, t5825: f64, t605: f64, t606: f64, t6084: f64, t6405: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20188, t20191, t20195, t20204, t20211, t20214) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2557(t19456, t996, t1678, t4746, t1695, t5015, t3269, t6343, t994, t19462, t378, t4772);
        let (t20215, t20219, t20228) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2558(t1079, t20214, t1096, t6258, t1000, t1073, t1076, t11201, t16302, t16362, t1652, t1680, t1696, t20188, t20191, t20195, t20204, t20211, t3047, t3063, t4743, t4752, t4935, t4947, t6235, t6259, t995);
        let (t20230, t20234) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2559(t19390, t19434, t20187, t20228, t1100, t1102, t19143, t19145, t19149, t19152, t19153, t19252, t19258, t19315, t19317, t19320, t19323, t19326, t19329, t19333, t19337, t19470, t19473, t19475, t198, t336, t5019, t5023, t5024);
        let (t20236, t20248) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2560(t30, t265, t393, t18884, t19141, t20234, t1106, t1468, t1469, t1704, t18280, t18281, t18892, t395, t4186, t45, t4560, t5028, t5824, t5825, t605, t606, t6084, t6405, t895, dens_threshold, rho0, zeta_threshold);
        let t20256 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2561(t18280);
    (t20188, t20191, t20195, t20204, t20211, t20215, t20219, t20230, t20236, t20248, t20256)
}
