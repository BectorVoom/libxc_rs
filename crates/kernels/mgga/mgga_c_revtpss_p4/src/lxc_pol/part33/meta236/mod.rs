//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1062;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1063;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1064;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1065;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta236<F: Float>(t378: F, t6305: F, t3304: F, t1089: F, t1668: F, t1678: F, t6299: F, t3318: F, t380: F, t6343: F, t1024: F, t1087: F, t1647: F, t1685: F, t1689: F, t1692: F, t3204: F, t3287: F, t3299: F, t3317: F, t342: F, t381: F, t4857: F, t4954: F, t6235: F, t6362: F, t6365: F, t6368: F, t6371: F, t1079: F, t1076: F, t1652: F, t1680: F, t1696: F, t3058: F, t386: F, t4747: F, t4752: F, t4778: F, t4935: F, t6245: F, t6251: F, t6259: F, t6345: F, t6351: F, t995: F, t1699: F, t1102: F, t198: F, t3336: F, t336: F, t6106: F, t6108: F, t6112: F, t6144: F, t6147: F, t6213: F, t6215: F, t6217: F, t6221: F, t6225: F, t6229: F, t30: F, t265: F, t393: F, t6084: F, t1468: F, t1469: F, t1587: F, t1704: F, t395: F, t45: F, t5824: F, t5825: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6375, t6379, t6383, t6386, t6389, t6392) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1062::<F>(t378, t6305, t3304, t1089, t1668, t1678, t6299, t3318, t380, t6343, t1024, t1087, t1647, t1685, t1689, t1692, t3204, t3287, t3299, t3317, t342, t381, t4857, t4954, t6235, t6362, t6365, t6368, t6371);
        let (t6393, t6396) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1063::<F>(t1079, t6392, t1076, t1647, t1652, t1680, t1696, t3058, t342, t386, t4747, t4752, t4778, t4935, t6235, t6245, t6251, t6259, t6345, t6351, t995);
        let (t6400, t6404) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1064::<F>(t1699, t1102, t198, t3336, t336, t6106, t6108, t6112, t6144, t6147, t6213, t6215, t6217, t6221, t6225, t6229, t6396);
        let (t6405, t6412) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1065::<F>(t30, t265, t393, t6084, t6404, t1468, t1469, t1587, t1704, t395, t45, t5824, t5825, dens_threshold, rho0, zeta_threshold);
        let t6416 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1066::<F>(t5824);
    (t6375, t6379, t6383, t6386, t6389, t6392, t6393, t6396, t6400, t6405, t6412, t6416)
}
