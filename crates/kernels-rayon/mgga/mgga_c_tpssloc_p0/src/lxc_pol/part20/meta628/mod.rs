//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2278;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2279;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2280;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta628(t39549: f64, t40779: f64, t40784: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t46303: f64, t46309: f64, t46311: f64, t46313: f64, t46314: f64, t46315: f64, t46318: f64, t46319: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t46331: f64, t46334: f64, t46336: f64, t46338: f64, t46339: f64, t46345: f64, t46349: f64, t46353: f64, t46355: f64, t46361: f64, t46367: f64, t46370: f64, t46372: f64, t39658: f64, t41254: f64, t41258: f64, t41262: f64, t46377: f64, t46384: f64, t46385: f64, t46386: f64, t46389: f64, t46432: f64, t46434: f64, t46436: f64, t46438: f64, t46439: f64, t46444: f64, t46446: f64, t46449: f64, t41282: f64, t4205: f64, t9926: f64, t1462: f64, t40709: f64, t13126: f64, t2250: f64, t4194: f64, t4195: f64, t9258: f64, t12890: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t47145 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2278(t39549, t40779, t40784, t40790, t40793, t40797, t40799, t40801, t40803, t46303, t46309, t46311, t46313, t46314, t46315, t46318, t46319);
        let t47146 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2279(t39563, t39585, t39590, t39593, t46331, t46334, t46336, t46338, t46339, t46345, t46349, t46353, t46355, t46361, t46367, t46370, t46372);
        let t47148 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2280(t39658, t41254, t41258, t41262, t46377, t46384, t46385, t46386, t46389, t46432, t46434, t46436, t46438, t46439, t46444, t46446, t46449);
        let (t47149, t47151, t47153, t47156, t47159, t47160) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2281(t41282, t4205, t9926, t1462, t40709, t13126, t2250, t4194, t4195, t9258, t12890, t751);
    (t47145, t47146, t47148, t47149, t47151, t47153, t47156, t47159, t47160)
}
