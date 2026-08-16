//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta189 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk892;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk893;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk894;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk895;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk896;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk897;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk898;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta189(t1079: f64, t5015: f64, t1000: f64, t1073: f64, t1076: f64, t1097: f64, t1647: f64, t1652: f64, t1680: f64, t1696: f64, t3047: f64, t3052: f64, t3058: f64, t3063: f64, t3264: f64, t342: f64, t386: f64, t4743: f64, t4747: f64, t4752: f64, t4758: f64, t4764: f64, t4773: f64, t4778: f64, t4932: f64, t4935: f64, t4941: f64, t4947: f64, t989: f64, t995: f64, t198: f64, t336: f64, t1699: f64, t3336: f64, t1100: f64, t1102: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4716: f64, t4718: f64, t4721: f64, t4723: f64, t4727: f64, t4731: f64, t4736: f64, t30: f64, t265: f64, t393: f64, t4560: f64, t1106: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t395: f64, t4186: f64, t45: f64, t4568: f64, t605: f64, t606: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1716: f64, t689: f64, t3362: f64, t3360: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5016 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk892(t1079, t5015);
        let t5019 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk893(t1000, t1073, t1076, t1097, t1647, t1652, t1680, t1696, t3047, t3052, t3058, t3063, t3264, t342, t386, t4743, t4747, t4752, t4758, t4764, t4773, t4778, t4932, t4935, t4941, t4947, t5016, t989, t995);
        let t5023 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk894(t198, t336);
        let (t5024, t5027) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk895(t1699, t3336, t1100, t1102, t198, t336, t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736, t5019, t5023);
        let (t5028, t5035) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk896(t30, t265, t393, t4560, t5027, t1106, t1468, t1469, t1587, t1704, t395, t4186, t45, t4568, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
        let t5044 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk897(t1716, t689);
        let (t5046, t5047) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk898(t1469, t3362, t606);
        let (t5048, t5049) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk899(t3360, t5047, t128);
    (t5016, t5019, t5023, t5024, t5028, t5035, t5044, t5046, t5047, t5048, t5049)
}
