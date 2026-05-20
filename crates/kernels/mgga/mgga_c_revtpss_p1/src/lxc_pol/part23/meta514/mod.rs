//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2017;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2018;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta514<F: Float>(t17235: F, t19661: F, t1042: F, t1235: F, t1238: F, t1252: F, t1261: F, t17505: F, t17569: F, t21063: F, t21085: F, t21088: F, t21091: F, t21095: F, t21102: F, t21107: F, t3667: F, t5279: F, t5320: F, t5327: F, t5384: F, t6647: F, t1248: F, t3604: F, t6688: F, t3720: F, t20266: F, t5312: F, t17475: F, t20293: F, t20318: F, t5308: F, t20310: F, t20306: F, t1260: F, t6601: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21110, t21111, t21114) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2017::<F>(t17235, t19661, t1042, t1235, t1238, t1252, t1261, t17505, t17569, t21063, t21085, t21088, t21091, t21095, t21102, t21107, t3667, t5279, t5320, t5327, t5384, t6647);
        let t21119 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2018::<F>(t1248, t3604);
        let (t21120, t21121, t21126, t21129, t21134, t21137, t21140, t21143) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2019::<F>(t21119, t6688, t3720, t20266, t5312, t17475, t20293, t20318, t5308, t20310, t20306, t1260, t6601);
    (t21110, t21111, t21114, t21119, t21120, t21121, t21126, t21129, t21134, t21137, t21140, t21143)
}
