//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 937/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk937<F: Float>(t1333: F, t2187: F, t2190: F, t1359: F, t1371: F, t2246: F, t2285: F, t2307: F, t2315: F, t3386: F, t3399: F, t3419: F, t6673: F, t6729: F, t821: F, t840: F, t849: F, t8869: F, t8901: F, t8905: F, t8908: F, t8910: F, t8911: F, t8916: F) -> (F, F, F) {
    let t8923 = t1333 * t2187;
    let t8925 = 2.0 * t8923 * t2190;
    let t8926 = 0.5848223622634646207e0 * t840 * t8869 + 1.0 * t6673 * t1359 + 2.0 * t2246 * t3386 + 1.0 * t821 * t8901 - t8905 - t8908 - t8910 + 0.11696447245269292414e1 * t8911 * t849 + 0.5848223622634646207e0 * t3399 * t2307 + 0.17315859105681463759e2 * t8916 * t2315 + 0.5848223622634646207e0 * t6729 * t1371 + 0.11696447245269292414e1 * t2285 * t3419 + t8925;
    (t8923, t8925, t8926)
}
