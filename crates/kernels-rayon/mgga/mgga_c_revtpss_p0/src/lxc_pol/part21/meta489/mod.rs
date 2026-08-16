//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2073;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2074;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2075;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2076;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2077;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta489(t15191: f64, t15197: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11334: f64, t11338: f64, t11339: f64, t11366: f64, t11368: f64, t15221: f64, t15230: f64, t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15127: f64, t15132: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64, t15189: f64, t15195: f64, t15200: f64, t15435: f64, t15450: f64, t935: f64, t915: f64, t15125: f64, t11560: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t324: f64, t11534: f64, t291: f64, t11399: f64, t15406: f64, t15413: f64, t15418: f64, t15420: f64, t15423: f64, t15425: f64, t15427: f64, t1622: f64, t2938: f64, t2963: f64, t2971: f64, t2989: f64, t4647: f64, t4670: f64, t15262: f64, t15348: f64, t15403: f64, t300: f64, t3007: f64, t4724: f64, t981: f64, t3022: f64, t4734: f64, t3011: f64, t4707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15457, t15459, t15472) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2073(t15191, t15197, t11134, t11136, t11138, t11140, t11334, t11338, t11339, t11366, t11368, t15221, t15230);
        let t15474 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2074(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15127, t15132, t15178, t15181, t15184, t15187, t15189, t15195, t15200, t15435, t15450, t15457, t15459, t15472);
        let (t15475, t15477, t15494) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2075(t15474, t935, t915, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11560, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15495, t15513) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2076(t15494, t324, t15125, t15191, t11134, t11136, t11138, t11140, t11534, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15515, t15516) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2077(t15513, t291, t11399, t15406, t15413, t15418, t15420, t15423, t15425, t15427, t15477, t15495, t1622, t2938, t2963, t2971, t2989, t4647, t4670);
        let (t15519, t15520, t15522, t15524, t15525) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2078(t15262, t15348, t15403, t15516, t300, t3007, t4724, t981, t3022, t4734, t3011, t4707);
    (t15474, t15475, t15477, t15494, t15495, t15513, t15515, t15519, t15520, t15522, t15524, t15525)
}
