//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1048;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1049;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1050;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta242(t2408: f64, t2411: f64, t262: f64, t775: f64, t10566: f64, t10568: f64, t10570: f64, t10575: f64, t10577: f64, t10580: f64, t10582: f64, t10584: f64, t2403: f64, t2430: f64, t4541: f64, t9514: f64, t9517: f64, t9521: f64, t10586: f64, t10589: f64, t10592: f64, t10594: f64, t10596: f64, t10598: f64, t10602: f64, t10604: f64, t10607: f64, t10609: f64, t10611: f64, t10614: f64, t9524: f64, t9542: f64, t10493: f64, t11082: f64, t1100: f64, t3333: f64, t3335: f64, t389: f64, t2918: f64, t936: f64, t2874: f64, t2926: f64, t934: f64, t2924: f64, t1077: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11084, t11092) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1048(t2408, t2411, t262, t775, t10566, t10568, t10570, t10575, t10577, t10580, t10582, t10584, t2403, t2430, t4541, t9514, t9517, t9521);
        let t11093 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1049(t10586, t10589, t10592, t10594, t10596, t10598, t10602, t10604, t10607, t10609, t10611, t10614, t9524, t9542);
        let t11095 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1050(t10493, t11082, t11092, t11093);
        let (t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11121) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1051(t1100, t3333, t3335, t389, t2918, t936, t2874, t2926, t934, t2924, t1077, t225);
    (t11084, t11095, t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11121)
}
