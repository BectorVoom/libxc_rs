//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2278;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2279;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2280;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta628<F: Float>(t39549: F, t40779: F, t40784: F, t40790: F, t40793: F, t40797: F, t40799: F, t40801: F, t40803: F, t46303: F, t46309: F, t46311: F, t46313: F, t46314: F, t46315: F, t46318: F, t46319: F, t39563: F, t39585: F, t39590: F, t39593: F, t46331: F, t46334: F, t46336: F, t46338: F, t46339: F, t46345: F, t46349: F, t46353: F, t46355: F, t46361: F, t46367: F, t46370: F, t46372: F, t39658: F, t41254: F, t41258: F, t41262: F, t46377: F, t46384: F, t46385: F, t46386: F, t46389: F, t46432: F, t46434: F, t46436: F, t46438: F, t46439: F, t46444: F, t46446: F, t46449: F, t41282: F, t4205: F, t9926: F, t1462: F, t40709: F, t13126: F, t2250: F, t4194: F, t4195: F, t9258: F, t12890: F, t751: F) -> (F, F, F, F, F, F, F, F, F) {
        let t47145 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2278::<F>(t39549, t40779, t40784, t40790, t40793, t40797, t40799, t40801, t40803, t46303, t46309, t46311, t46313, t46314, t46315, t46318, t46319);
        let t47146 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2279::<F>(t39563, t39585, t39590, t39593, t46331, t46334, t46336, t46338, t46339, t46345, t46349, t46353, t46355, t46361, t46367, t46370, t46372);
        let t47148 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2280::<F>(t39658, t41254, t41258, t41262, t46377, t46384, t46385, t46386, t46389, t46432, t46434, t46436, t46438, t46439, t46444, t46446, t46449);
        let (t47149, t47151, t47153, t47156, t47159, t47160) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2281::<F>(t41282, t4205, t9926, t1462, t40709, t13126, t2250, t4194, t4195, t9258, t12890, t751);
    (t47145, t47146, t47148, t47149, t47151, t47153, t47156, t47159, t47160)
}
