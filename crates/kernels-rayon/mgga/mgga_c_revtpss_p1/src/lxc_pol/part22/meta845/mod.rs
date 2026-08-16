//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta845 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2980;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2981;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta845(t14066: f64, t545: f64, t689: f64, t869: f64, t1398: f64, t14141: f64, t14143: f64, t2434: f64, t10049: f64, t14145: f64, t1882: f64, t2482: f64, t14230: f64, t2782: f64, t46456: f64, t1385: f64, t14155: f64, t1432: f64, t2470: f64, t1892: f64, t4056: f64, t4086: f64, t543: f64, t10069: f64, t14225: f64, t10013: f64, t14224: f64, t48073: f64, t4100: f64, t49213: f64, t10136: f64, t14114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49252, t49256, t49260) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2980(t14066, t545, t689, t869, t1398, t14141, t14143, t2434, t10049, t14145, t1882, t2482);
        let (t49263, t49268, t49273, t49283) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2981(t14230, t2782, t46456, t1385, t14066, t14155, t1432, t2470, t1892, t4056, t4086, t543);
        let (t49289, t49296, t49308, t49313, t49321) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2982(t10069, t14225, t10013, t14224, t2782, t48073, t543, t4100, t4086, t49213, t10136, t14114);
    (t49252, t49256, t49260, t49263, t49268, t49273, t49283, t49289, t49296, t49308, t49313, t49321)
}
