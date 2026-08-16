//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1575;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1576;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1577;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta442(t127: f64, t371: f64, t6337: f64, t3205: f64, t6276: f64, t1025: f64, t4845: f64, t4858: f64, t3172: f64, t6307: f64, t3150: f64, t4820: f64, t4879: f64, t11947: f64, t15745: f64, t16134: f64, t16160: f64, t16190: f64, t1665: f64, t1671: f64, t3188: f64, t6327: f64, t6339: f64, t1592: f64, t999: f64, t1045: f64, t15691: f64, t1066: f64, t18946: f64, t247: f64, t11725: f64, t6092: f64, t1063: f64, t3109: f64, t6100: f64, t19572: f64, t4894: f64, t3117: f64, t4900: f64, t11774: f64, t15926: f64, t3106: f64, t4892: f64, t4899: f64, t4912: f64, t6323: f64, t6331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20016, t20017, t20020, t20021, t20025, t20029, t20030, t20034) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1575(t127, t371, t6337, t3205, t6276, t1025, t4845, t4858, t3172, t6307, t3150, t4820, t4879);
        let t20036 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1576(t11947, t15745, t16134, t16160, t16190, t1665, t1671, t20017, t20021, t20025, t20030, t20034, t3188, t6327, t6339);
        let (t20040, t20046, t20050, t20051, t20054) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1577(t1592, t999, t1045, t15691, t1066, t18946, t247, t11725, t6092, t1063, t3109, t6100);
        let (t20066, t20070, t20073) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1578(t1063, t20054, t19572, t4894, t3117, t4900, t11774, t15926, t20040, t20046, t20051, t3106, t3188, t4892, t4899, t4912, t6323, t6327, t6331);
    (t20016, t20020, t20029, t20036, t20040, t20046, t20050, t20054, t20066, t20070, t20073)
}
