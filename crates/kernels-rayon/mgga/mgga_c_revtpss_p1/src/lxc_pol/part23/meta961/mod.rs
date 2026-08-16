//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta961 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3245;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3246;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3247;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3248;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta961(t30: f64, t46281: f64, t46286: f64, t5824: f64, t605: f64, t580: f64, t1344: f64, t13687: f64, t13690: f64, t18280: f64, t21944: f64, t2255: f64, t22670: f64, t22769: f64, t3874: f64, t46310: f64, t5574: f64, t76396: f64, zeta_threshold: f64, t33: f64, t1113: f64, t6416: f64, t1348: f64, t13701: f64, t13704: f64, t20256: f64, t21956: f64, t22778: f64, t22783: f64, t3881: f64, t46328: f64, t5582: f64, t81123: f64, t1424: f64, t14299: f64, t22415: f64, t22971: f64, t23043: f64, t4071: f64, t4076: f64, t46359: f64, t47764: f64, t47772: f64, t47781: f64, t47785: f64, t47786: f64, t5715: f64, t5774: f64, t6896: f64, t6918: f64, t73587: f64, t73590: f64, t73593: f64, t73598: f64, t23042: f64, t3915: f64, t686: f64, t72: f64, t22970: f64, t9680: f64, t46368: f64, t46369: f64, t46378: f64, t46385: f64, t46388: f64, t47800: f64, t47802: f64, t47806: f64, t47814: f64, t47835: f64, t47838: f64, t47839: f64, t73623: f64, t73627: f64, t22453: f64, t49471: f64, t47474: f64, t47478: f64, t47487: f64, t47495: f64, t47497: f64, t47845: f64, t47858: f64, t47860: f64, t47863: f64, t73641: f64, t73647: f64, t73652: f64, t73656: f64, t73662: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85390, t85391, t85406, t85409, t85420) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3245(t30, t46281, t46286, t5824, t605, t580, t1344, t13687, t13690, t18280, t21944, t2255, t22670, t22769, t3874, t46310, t5574, t76396, zeta_threshold);
        let (t85426, t85429, t85440) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3246(t33, t1113, t6416, t580, t1348, t13701, t13704, t20256, t21956, t2255, t22778, t22783, t3881, t46328, t5582, t81123, zeta_threshold);
        let (t85442, t85466) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3247(t85420, t85440, t1424, t14299, t22415, t22971, t23043, t4071, t4076, t46359, t47764, t47772, t47781, t47785, t47786, t5715, t5774, t6896, t6918, t73587, t73590, t73593, t73598);
        let t85482 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3248(t23042, t3915, t686, t72, t22970, t9680, t46368, t46369, t46378, t46385, t46388, t47800, t47802, t47806, t47814, t47835, t47838, t47839, t73623, t73627);
        let t85498 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3249(t22453, t49471, t47474, t47478, t47487, t47495, t47497, t47845, t47858, t47860, t47863, t73641, t73647, t73652, t73656, t73662);
    (t85390, t85391, t85406, t85409, t85426, t85429, t85442, t85466, t85482, t85498)
}
