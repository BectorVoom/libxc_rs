//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1398/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1398<F: Float>(t1632: F, t551: F, t566: F, t9880: F, t7383: F, t8873: F, t2155: F, t32666: F, t8773: F, t910: F, t2148: F, t22790: F, t8783: F, t938: F, t22868: F, t10359: F, t277: F) -> (F, F, F, F, F, F) {
    let t33911 = t566 * t551 * t1632 * t9880;
    let t33915 = t7383 * t8873;
    let t33922 = t2155 * t32666;
    let t33925 = t8773 * t910;
    let t33927 = t22790 * t2148 * t33925;
    let t33929 = t8783 * t938;
    let t33931 = t22868 * t2148 * t33929;
    let t33933 = t277 * t10359;
    (t33911, t33915, t33922, t33927, t33931, t33933)
}
