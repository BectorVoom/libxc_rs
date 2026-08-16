//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta688 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2507;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta688(t12984: f64, t3667: f64, t12976: f64, t3678: f64, t12963: f64, t1235: f64, t127: f64, t12970: f64, t371: f64, t126: f64, t13099: f64, t12257: f64, t1261: f64, t247: f64, t12879: f64, t3372: f64, t3368: f64, t1222: f64, t12287: f64, t17240: f64, t12881: f64, t3647: f64, t1224: f64, t12268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44884, t44886, t44888, t44892, t44898) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2507(t12984, t3667, t12976, t3678, t12963, t1235, t127, t12970, t371, t126, t13099, t12257, t1261, t247);
        let (t44902, t44906, t44912, t44917, t44919) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2508(t1261, t12879, t247, t3372, t3368, t1222, t12287, t17240, t12881, t3647, t1224, t12268);
    (t44884, t44886, t44888, t44892, t44898, t44902, t44906, t44912, t44917, t44919)
}
