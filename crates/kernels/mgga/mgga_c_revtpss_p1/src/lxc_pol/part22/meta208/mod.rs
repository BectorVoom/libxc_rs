//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta208 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1322;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1323;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1324;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1325;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1326;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1327;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1328;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1329;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1330;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta208<F: Float>(t1000: F, t1073: F, t1076: F, t1097: F, t1647: F, t1652: F, t1680: F, t1696: F, t3047: F, t3052: F, t3058: F, t3063: F, t3264: F, t342: F, t386: F, t4743: F, t4747: F, t4752: F, t4758: F, t4764: F, t4773: F, t4778: F, t4932: F, t4935: F, t4941: F, t4947: F, t5016: F, t989: F, t995: F, t198: F, t336: F, t1699: F, t3336: F, t1100: F, t1102: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4716: F, t4718: F, t4721: F, t4723: F, t4727: F, t4731: F, t4736: F, t30: F, t265: F, t393: F, t4560: F, t1106: F, t1468: F, t1469: F, t1587: F, t1704: F, t395: F, t4186: F, t45: F, t4568: F, t605: F, t606: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1716: F, t689: F, t3362: F, t3360: F, t128: F, t3367: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5019 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1322::<F>(t1000, t1073, t1076, t1097, t1647, t1652, t1680, t1696, t3047, t3052, t3058, t3063, t3264, t342, t386, t4743, t4747, t4752, t4758, t4764, t4773, t4778, t4932, t4935, t4941, t4947, t5016, t989, t995);
        let t5023 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1323::<F>(t198, t336);
        let (t5024, t5027) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1324::<F>(t1699, t3336, t1100, t1102, t198, t336, t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736, t5019, t5023);
        let (t5028, t5035) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1325::<F>(t30, t265, t393, t4560, t5027, t1106, t1468, t1469, t1587, t1704, t395, t4186, t45, t4568, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
        let t5044 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1326::<F>(t1716, t689);
        let t5046 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1327::<F>(t1469, t3362);
        let t5047 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1328::<F>(t5046, t606);
        let (t5048, t5049) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1329::<F>(t3360, t5047, t128);
        let t5051 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1330::<F>(t1469, t3367);
        let t5052 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1331::<F>(t5051, t606);
    (t5019, t5023, t5024, t5028, t5035, t5044, t5046, t5047, t5048, t5049, t5051, t5052)
}
