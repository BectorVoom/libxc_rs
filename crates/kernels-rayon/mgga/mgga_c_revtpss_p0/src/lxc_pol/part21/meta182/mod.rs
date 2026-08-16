//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1122;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1123;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1124;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta182(t1514: f64, t625: f64, t1513: f64, t2339: f64, t665: f64, t1504: f64, t2349: f64, t658: f64, t100: f64, t2: f64, t580: f64, t1509: f64, t2357: f64, t661: f64, t108: f64, t105: f64, t1505: f64, t1507: f64, t656: f64, t662: f64, t97: f64, t114: f64, t655: f64, t2335: f64, t2336: f64, t69: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4261, t4263, t4264, t4269, t4270, t4273, t4274, t4279) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1122(t1514, t625, t1513, t2339, t665, t1504, t2349, t658, t100, t2, t580, t1509, t2357);
        let (t4283, t4287) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1123(t4279, t661, t108, t2, t580, t105, t1505, t1507, t4270, t4274, t656, t662, t97);
        let (t4288, t4292) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1124(t114, t4287, t655, t2335, t2336, t4261, t4264, t69);
        let t4293 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1125(t4292, t508);
    (t4263, t4264, t4269, t4270, t4273, t4274, t4279, t4283, t4287, t4288, t4292, t4293)
}
