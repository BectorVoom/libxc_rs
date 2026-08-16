//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta863 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3015;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta863(t10811: f64, t14707: f64, t14874: f64, t14673: f64, t40731: f64, t40593: f64, t4447: f64, t4462: f64, t10760: f64, t40763: f64, t4353: f64, t1559: f64, t775: f64, t40834: f64, t854: f64, t14587: f64, t2735: f64, t40798: f64, t826: f64, t10777: f64, t10779: f64, t2749: f64, t50412: f64, t14686: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50600, t50602, t50604, t50606, t50608, t50611, t50613) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3015(t10811, t14707, t14874, t14673, t40731, t40593, t4447, t4462, t10760, t40763, t4353, t1559, t775);
        let (t50615, t50619, t50628, t50632) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3016(t40834, t50613, t854, t14587, t2735, t40798, t826, t10777, t10779, t2749, t50412, t14686, t837);
    (t50600, t50602, t50604, t50606, t50608, t50611, t50613, t50615, t50619, t50628, t50632)
}
