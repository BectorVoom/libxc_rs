//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta846 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2983;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta846(t1882: f64, t2482: f64, t4104: f64, t4118: f64, t1398: f64, t2782: f64, t4086: f64, t543: f64, t5710: f64, t1897: f64, t40317: f64, t10111: f64, t22: f64, t5759: f64, t49146: f64, t4100: f64, t48475: f64, t47423: f64, t5741: f64, t3923: f64, t48105: f64, t47371: f64, t10026: f64, t14141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49325, t49346, t49354, t49361) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2983(t1882, t2482, t4104, t4118, t1398, t2782, t4086, t543, t5710, t1897, t40317, t10111, t22, t5759);
        let (t49376, t49378, t49382, t49386, t49395, t49399) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2984(t49146, t543, t2782, t4100, t48475, t47423, t5741, t3923, t48105, t47371, t10026, t14141);
    (t49325, t49346, t49354, t49361, t49376, t49378, t49382, t49386, t49395, t49399)
}
