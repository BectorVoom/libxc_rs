//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2626;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta666<F: Float>(t21082: F, t482: F, t371: F, t372: F, t5323: F, t5362: F, t12772: F, t6639: F, t3625: F, t1263: F, t6573: F, t1122: F, t1042: F, t1038: F, t6593: F, t1244: F, t1241: F, t5273: F, t5292: F, t17235: F, t19661: F, t1235: F, t1238: F, t1252: F, t1261: F, t17505: F, t17569: F, t21063: F, t3667: F, t5279: F, t5320: F, t5327: F, t5384: F, t6647: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21083, t21085, t21088, t21090, t21091, t21093, t21094) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2626::<F>(t21082, t482, t371, t372, t5323, t5362, t12772, t6639, t3625, t1263, t6573, t1122);
        let (t21095, t21101, t21102, t21107, t21110, t21111, t21114) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2627::<F>(t1042, t21094, t1038, t6593, t1244, t1241, t5273, t5292, t17235, t19661, t1235, t1238, t1252, t1261, t17505, t17569, t21063, t21085, t21088, t21091, t3667, t5279, t5320, t5327, t5384, t6647);
    (t21083, t21085, t21090, t21093, t21094, t21095, t21101, t21102, t21107, t21110, t21111, t21114)
}
