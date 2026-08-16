//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta882 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3055;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta882(t10504: f64, t136: f64, t2457: f64, t4533: f64, t14481: f64, t2782: f64, t861: f64, t11050: f64, t14987: f64, t14473: f64, t9303: f64, t41017: f64, t4481: f64, t14477: f64, t2435: f64, t14978: f64, t2465: f64, t686: f64, t72: f64, t14480: f64, t252: f64, t2828: f64, t10073: f64, t14482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51726, t51729, t51731, t51733, t51739) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3055(t10504, t136, t2457, t4533, t14481, t2782, t861, t11050, t14987, t14473, t9303, t41017, t4481);
        let (t51741, t51746, t51750, t51756) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3056(t14477, t2435, t14978, t2465, t686, t72, t14480, t252, t2782, t2828, t10073, t14482);
    (t51726, t51729, t51731, t51733, t51739, t51741, t51746, t51750, t51756)
}
