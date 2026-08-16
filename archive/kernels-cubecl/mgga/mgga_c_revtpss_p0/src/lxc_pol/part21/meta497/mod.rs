//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta497 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2098;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2099;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2100;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2101;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta497<F: Float>(t372: F, t4801: F, t4181: F, t4786: F, t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F, t11940: F, t1651: F, t3059: F, t3116: F, t11672: F, t11675: F, t11712: F, t11774: F, t15684: F, t15689: F, t15693: F, t15697: F, t15700: F, t3101: F, t3106: F, t3130: F, t4788: F, t4831: F, t4834: F, t3111: F, t11788: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15701, t15702, t15703, t15707) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2098::<F>(t372, t4801, t4181, t4786, t1062, t4857);
        let (t15711, t15712, t15716) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2099::<F>(t11986, t1592, t247, t1063, t1062, t11940);
        let t15717 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2100::<F>(t1651, t3059);
        let (t15719, t15722) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2101::<F>(t15717, t247, t3116, t11672, t11675, t11712, t11774, t15684, t15689, t15693, t15697, t15700, t15703, t15707, t15712, t15716, t3101, t3106, t3130, t4788, t4831, t4834);
        let (t15724, t15725) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2102::<F>(t3111, t4834, t1062, t11788);
    (t15701, t15702, t15703, t15707, t15711, t15716, t15717, t15719, t15722, t15724, t15725)
}
