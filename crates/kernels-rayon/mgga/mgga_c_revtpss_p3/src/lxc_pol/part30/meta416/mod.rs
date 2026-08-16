//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1566;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1567;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1568;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1569;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1570;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1571;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1572;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta416(t1614: f64, t2967: f64, t1626: f64, t2986: f64, t4587: f64, t914: f64, t936: f64, t2919: f64, t4590: f64, t1596: f64, t2923: f64, t2927: f64, t11289: f64, t1610: f64, t2869: f64, t4632: f64, t15125: f64, t15168: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15163: f64, t15166: f64, t15170: f64, t15173: f64, t15191: f64, t15197: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11334: f64, t11338: f64, t11339: f64, t11366: f64, t11368: f64, t15221: f64, t15230: f64, t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15127: f64, t15132: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64, t15189: f64, t15195: f64, t15200: f64, t935: f64, t915: f64, t11560: f64, t324: f64, t11534: f64, t291: f64, t11399: f64, t1622: f64, t2938: f64, t2963: f64, t2971: f64, t2989: f64, t4647: f64, t4670: f64, t15262: f64, t15348: f64, t15403: f64, t300: f64, t3007: f64, t4724: f64, t981: f64, t3022: f64, t4734: f64, t3011: f64, t4707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15406, t15413, t15418, t15420, t15423) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1566(t1614, t2967, t1626, t2986, t4587, t914, t936, t2919, t4590, t1596, t2923, t2927);
        let (t15425, t15427, t15435, t15450) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1567(t11289, t1610, t2869, t4632, t15125, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
        let (t15457, t15459, t15472) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1568(t15191, t15197, t11134, t11136, t11138, t11140, t11334, t11338, t11339, t11366, t11368, t15221, t15230);
        let t15474 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1569(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15127, t15132, t15178, t15181, t15184, t15187, t15189, t15195, t15200, t15435, t15450, t15457, t15459, t15472);
        let (t15477, t15494) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1570(t15474, t935, t915, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11560, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15495, t15513) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1571(t15494, t324, t15125, t15191, t11134, t11136, t11138, t11140, t11534, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15515, t15516) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1572(t15513, t291, t11399, t15406, t15413, t15418, t15420, t15423, t15425, t15427, t15477, t15495, t1622, t2938, t2963, t2971, t2989, t4647, t4670);
        let (t15519, t15522, t15524, t15525) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1573(t15262, t15348, t15403, t15516, t300, t3007, t4724, t981, t3022, t4734, t3011, t4707);
    (t15418, t15420, t15423, t15425, t15427, t15477, t15495, t15515, t15519, t15522, t15524, t15525)
}
