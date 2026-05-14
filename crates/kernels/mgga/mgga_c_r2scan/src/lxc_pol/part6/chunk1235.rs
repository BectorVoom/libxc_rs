//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1235/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1235<F: Float>(t20379: F, t2155: F, t571: F, t572: F, t6311: F, t19790: F, t495: F, t19789: F, t1550: F, t538: F, t6191: F, t6194: F, t164: F, t277: F, t783: F, t785: F) -> (F, F, F, F, F) {
    let t22944 = t2155 * t20379;
    let t22947 = t571 * t572 * t6311;
    let t22948 = t19790 * t495;
    let t22950 = t22947 * t19789 * t22948;
    let t22954 = t6191 * t538 * t1550 * t6194;
    let t22959 = 0.83631605379377467466e1 * t783 * t785 * t164 * t277;
    (t22944, t22947, t22950, t22954, t22959)
}
