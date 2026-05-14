//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 960/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk960<F: Float>(t361: F, t9266: F, t1422: F, t1434: F, t2533: F, t2540: F, t2555: F, t2563: F, t2572: F, t2579: F, t2602: F, t3527: F, t3547: F, t3580: F, t7099: F, t7154: F, t9205: F, t9210: F, t9242: F, t9245: F, t9248: F, t9255: F, t9260: F, t979: F, t988: F) -> (F, F) {
    let t9268 = 0.621814e-1 * t9266 * t361;
    let t9269 = 2.0 * t9205 * t988 + 1.0 * t3527 * t2555 + 0.32163958997385070134e2 * t9210 * t2563 + 1.0 * t7154 * t1422 + 2.0 * t2533 * t3547 + 1.0 * t979 * t9242 - 2.0 * t9245 * t2540 + 0.17315859105681463759e2 * t9248 * t2602 + 0.5848223622634646207e0 * t7099 * t1434 + 0.11696447245269292414e1 * t2572 * t3580 - 0.11696447245269292414e1 * t9255 * t2579 + t9260 + t9268;
    (t9268, t9269)
}
