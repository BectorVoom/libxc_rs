//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2166;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2167;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2168;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2169;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2170;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta471(t1651: f64, t3059: f64, t247: f64, t3116: f64, t11672: f64, t11675: f64, t11712: f64, t11774: f64, t15684: f64, t15689: f64, t15693: f64, t15697: f64, t15700: f64, t15703: f64, t15707: f64, t15712: f64, t15716: f64, t3101: f64, t3106: f64, t3130: f64, t4788: f64, t4831: f64, t4834: f64, t3111: f64, t1062: f64, t11788: f64, t3105: f64, t3204: f64, t11262: f64, t1670: f64, t1041: f64, t3172: f64, t4824: f64, t3127: f64, t3211: f64, t4845: f64, t1053: f64, t4857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t15717 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2166(t1651, t3059);
        let (t15719, t15722) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2167(t15717, t247, t3116, t11672, t11675, t11712, t11774, t15684, t15689, t15693, t15697, t15700, t15703, t15707, t15712, t15716, t3101, t3106, t3130, t4788, t4831, t4834);
        let (t15724, t15725) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2168(t3111, t4834, t1062, t11788);
        let t15728 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2169(t3105, t3204);
        let (t15731, t15732, t15734, t15736, t15744, t15745) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2170(t11262, t1670, t1041, t3172, t4824, t3127, t3211, t4845, t1053, t4857);
    (t15717, t15719, t15722, t15724, t15725, t15728, t15731, t15732, t15734, t15736, t15744, t15745)
}
