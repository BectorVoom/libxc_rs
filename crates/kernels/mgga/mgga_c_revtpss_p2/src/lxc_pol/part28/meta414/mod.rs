//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1568;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1569;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1570;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1571;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1572;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1573;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1574;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1575;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta414<F: Float>(t1614: F, t2967: F, t1626: F, t2986: F, t4587: F, t914: F, t936: F, t2919: F, t4590: F, t1596: F, t2923: F, t2927: F, t11289: F, t1610: F, t2869: F, t4632: F, t15125: F, t15168: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15163: F, t15166: F, t15170: F, t15173: F, t15191: F, t15197: F, t11134: F, t11136: F, t11138: F, t11140: F, t11334: F, t11338: F, t11339: F, t11366: F, t11368: F, t15221: F, t15230: F, t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15127: F, t15132: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15195: F, t15200: F, t935: F, t915: F, t11560: F, t324: F, t11534: F, t291: F, t11399: F, t1622: F, t2938: F, t2963: F, t2971: F, t2989: F, t4647: F, t4670: F, t15262: F, t15348: F, t15403: F, t300: F, t3007: F, t4724: F, t981: F, t3022: F, t4734: F, t3011: F, t4707: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15406, t15413, t15418, t15420, t15423) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1568::<F>(t1614, t2967, t1626, t2986, t4587, t914, t936, t2919, t4590, t1596, t2923, t2927);
        let (t15425, t15427, t15435, t15450) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1569::<F>(t11289, t1610, t2869, t4632, t15125, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
        let (t15457, t15459, t15472) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1570::<F>(t15191, t15197, t11134, t11136, t11138, t11140, t11334, t11338, t11339, t11366, t11368, t15221, t15230);
        let t15474 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1571::<F>(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15127, t15132, t15178, t15181, t15184, t15187, t15189, t15195, t15200, t15435, t15450, t15457, t15459, t15472);
        let (t15477, t15494) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1572::<F>(t15474, t935, t915, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11560, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15495, t15513) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1573::<F>(t15494, t324, t15125, t15191, t11134, t11136, t11138, t11140, t11534, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15515, t15516) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1574::<F>(t15513, t291, t11399, t15406, t15413, t15418, t15420, t15423, t15425, t15427, t15477, t15495, t1622, t2938, t2963, t2971, t2989, t4647, t4670);
        let (t15519, t15522, t15524, t15525) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1575::<F>(t15262, t15348, t15403, t15516, t300, t3007, t4724, t981, t3022, t4734, t3011, t4707);
    (t15418, t15420, t15423, t15425, t15427, t15477, t15495, t15515, t15519, t15522, t15524, t15525)
}
