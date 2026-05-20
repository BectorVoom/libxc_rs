//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1048;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1049;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1050;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta242<F: Float>(t2408: F, t2411: F, t262: F, t775: F, t10566: F, t10568: F, t10570: F, t10575: F, t10577: F, t10580: F, t10582: F, t10584: F, t2403: F, t2430: F, t4541: F, t9514: F, t9517: F, t9521: F, t10586: F, t10589: F, t10592: F, t10594: F, t10596: F, t10598: F, t10602: F, t10604: F, t10607: F, t10609: F, t10611: F, t10614: F, t9524: F, t9542: F, t10493: F, t11082: F, t1100: F, t3333: F, t3335: F, t389: F, t2918: F, t936: F, t2874: F, t2926: F, t934: F, t2924: F, t1077: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11084, t11092) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1048::<F>(t2408, t2411, t262, t775, t10566, t10568, t10570, t10575, t10577, t10580, t10582, t10584, t2403, t2430, t4541, t9514, t9517, t9521);
        let t11093 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1049::<F>(t10586, t10589, t10592, t10594, t10596, t10598, t10602, t10604, t10607, t10609, t10611, t10614, t9524, t9542);
        let t11095 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1050::<F>(t10493, t11082, t11092, t11093);
        let (t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11121) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1051::<F>(t1100, t3333, t3335, t389, t2918, t936, t2874, t2926, t934, t2924, t1077, t225);
    (t11084, t11095, t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11121)
}
