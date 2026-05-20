//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta961 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3245;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3246;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3247;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3248;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta961<F: Float>(t30: F, t46281: F, t46286: F, t5824: F, t605: F, t580: F, t1344: F, t13687: F, t13690: F, t18280: F, t21944: F, t2255: F, t22670: F, t22769: F, t3874: F, t46310: F, t5574: F, t76396: F, zeta_threshold: F, t33: F, t1113: F, t6416: F, t1348: F, t13701: F, t13704: F, t20256: F, t21956: F, t22778: F, t22783: F, t3881: F, t46328: F, t5582: F, t81123: F, t1424: F, t14299: F, t22415: F, t22971: F, t23043: F, t4071: F, t4076: F, t46359: F, t47764: F, t47772: F, t47781: F, t47785: F, t47786: F, t5715: F, t5774: F, t6896: F, t6918: F, t73587: F, t73590: F, t73593: F, t73598: F, t23042: F, t3915: F, t686: F, t72: F, t22970: F, t9680: F, t46368: F, t46369: F, t46378: F, t46385: F, t46388: F, t47800: F, t47802: F, t47806: F, t47814: F, t47835: F, t47838: F, t47839: F, t73623: F, t73627: F, t22453: F, t49471: F, t47474: F, t47478: F, t47487: F, t47495: F, t47497: F, t47845: F, t47858: F, t47860: F, t47863: F, t73641: F, t73647: F, t73652: F, t73656: F, t73662: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t85390, t85391, t85406, t85409, t85420) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3245::<F>(t30, t46281, t46286, t5824, t605, t580, t1344, t13687, t13690, t18280, t21944, t2255, t22670, t22769, t3874, t46310, t5574, t76396, zeta_threshold);
        let (t85426, t85429, t85440) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3246::<F>(t33, t1113, t6416, t580, t1348, t13701, t13704, t20256, t21956, t2255, t22778, t22783, t3881, t46328, t5582, t81123, zeta_threshold);
        let (t85442, t85466) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3247::<F>(t85420, t85440, t1424, t14299, t22415, t22971, t23043, t4071, t4076, t46359, t47764, t47772, t47781, t47785, t47786, t5715, t5774, t6896, t6918, t73587, t73590, t73593, t73598);
        let t85482 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3248::<F>(t23042, t3915, t686, t72, t22970, t9680, t46368, t46369, t46378, t46385, t46388, t47800, t47802, t47806, t47814, t47835, t47838, t47839, t73623, t73627);
        let t85498 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3249::<F>(t22453, t49471, t47474, t47478, t47487, t47495, t47497, t47845, t47858, t47860, t47863, t73641, t73647, t73652, t73656, t73662);
    (t85390, t85391, t85406, t85409, t85426, t85429, t85442, t85466, t85482, t85498)
}
