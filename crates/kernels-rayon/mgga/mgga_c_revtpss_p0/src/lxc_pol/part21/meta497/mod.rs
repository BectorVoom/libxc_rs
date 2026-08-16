//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2098;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2099;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2100;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2101;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta497(t372: f64, t4801: f64, t4181: f64, t4786: f64, t1062: f64, t4857: f64, t11986: f64, t1592: f64, t247: f64, t1063: f64, t11940: f64, t1651: f64, t3059: f64, t3116: f64, t11672: f64, t11675: f64, t11712: f64, t11774: f64, t15684: f64, t15689: f64, t15693: f64, t15697: f64, t15700: f64, t3101: f64, t3106: f64, t3130: f64, t4788: f64, t4831: f64, t4834: f64, t3111: f64, t11788: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15701, t15702, t15703, t15707) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2098(t372, t4801, t4181, t4786, t1062, t4857);
        let (t15711, t15712, t15716) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2099(t11986, t1592, t247, t1063, t1062, t11940);
        let t15717 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2100(t1651, t3059);
        let (t15719, t15722) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2101(t15717, t247, t3116, t11672, t11675, t11712, t11774, t15684, t15689, t15693, t15697, t15700, t15703, t15707, t15712, t15716, t3101, t3106, t3130, t4788, t4831, t4834);
        let (t15724, t15725) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2102(t3111, t4834, t1062, t11788);
    (t15701, t15702, t15703, t15707, t15711, t15716, t15717, t15719, t15722, t15724, t15725)
}
