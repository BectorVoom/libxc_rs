//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1319;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1320;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1321;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1322;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1323;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta363<F: Float>(t39756: F, t39760: F, t39764: F, t39767: F, t39770: F, t39773: F, t39775: F, t39778: F, t39780: F, t39783: F, t39786: F, t268: F, t681: F, t702: F, t793: F, t215: F, t2564: F, t2567: F, t2566: F, t2576: F, t9311: F, t9313: F, t2580: F, t2583: F, t130: F, t39525: F, t2563: F, t45: F, t2495: F, t9385: F, t2491: F, t744: F, t760: F, t2492: F, t2514: F, t9367: F, t9371: F, t200: F, t631: F, t10326: F, t10446: F, t10449: F, t2251: F, t2258: F, t2375: F, t39443: F, t39449: F, t39457: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39787, t39791) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1319::<F>(t39756, t39760, t39764, t39767, t39770, t39773, t39775, t39778, t39780, t39783, t39786, t268, t681, t702, t793);
        let t39795 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1320::<F>(t215, t2564, t2567, t268);
        let t39799 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1321::<F>(t2566, t2576, t9311, t9313);
        let t39807 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1322::<F>(t2580, t2583, t130, t39525);
        let t39813 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1323::<F>(t130, t2563, t2580, t39525, t9313);
        let (t39815, t39816, t39818, t39821, t39823, t39838) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1324::<F>(t45, t2495, t9385, t2491, t744, t760, t2492, t2514, t9367, t9371, t200, t631, t10326, t10446, t10449, t2251, t2258, t2375, t39443, t39449, t39457, t78, zeta_threshold);
    (t39787, t39791, t39795, t39799, t39807, t39813, t39815, t39816, t39818, t39821, t39823, t39838)
}
