//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1274/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1274<F: Float>(t46598: F, t13440: F, t3781: F, t850: F, t860: F, t1134: F, t49841: F, t1123: F, t12381: F, t339: F, t46615: F, t2080: F, t2083: F, t2085: F, t48997: F) -> (F, F, F, F, F, F) {
    let t50349 = F::new(7.0) / F::new(6.0) * t46598;
    let t50353 = t850 * t3781 * t13440 * t860 / F::new(32.0);
    let t50354 = t1134 * t49841;
    let t50362 = t850 * t1123 * t12381 * t339 * t860 / F::new(96.0);
    let t50363 = F::new(7.0) / F::new(36.0) * t46615;
    let t50368 = t2080 * t48997 * t2083 * t2085 * t860 / F::new(32.0);
    (t50349, t50353, t50354, t50362, t50363, t50368)
}
