//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta363 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1319;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1320;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1321;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1322;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1323;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta363(t39756: f64, t39760: f64, t39764: f64, t39767: f64, t39770: f64, t39773: f64, t39775: f64, t39778: f64, t39780: f64, t39783: f64, t39786: f64, t268: f64, t681: f64, t702: f64, t793: f64, t215: f64, t2564: f64, t2567: f64, t2566: f64, t2576: f64, t9311: f64, t9313: f64, t2580: f64, t2583: f64, t130: f64, t39525: f64, t2563: f64, t45: f64, t2495: f64, t9385: f64, t2491: f64, t744: f64, t760: f64, t2492: f64, t2514: f64, t9367: f64, t9371: f64, t200: f64, t631: f64, t10326: f64, t10446: f64, t10449: f64, t2251: f64, t2258: f64, t2375: f64, t39443: f64, t39449: f64, t39457: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39787, t39791) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1319(t39756, t39760, t39764, t39767, t39770, t39773, t39775, t39778, t39780, t39783, t39786, t268, t681, t702, t793);
        let t39795 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1320(t215, t2564, t2567, t268);
        let t39799 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1321(t2566, t2576, t9311, t9313);
        let t39807 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1322(t2580, t2583, t130, t39525);
        let t39813 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1323(t130, t2563, t2580, t39525, t9313);
        let (t39815, t39816, t39818, t39821, t39823, t39838) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1324(t45, t2495, t9385, t2491, t744, t760, t2492, t2514, t9367, t9371, t200, t631, t10326, t10446, t10449, t2251, t2258, t2375, t39443, t39449, t39457, t78, zeta_threshold);
    (t39787, t39791, t39795, t39799, t39807, t39813, t39815, t39816, t39818, t39821, t39823, t39838)
}
