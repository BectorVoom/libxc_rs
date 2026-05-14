//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 805/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk805<F: Float>(t506: F, t8629: F, t529: F, t552: F, t551: F, t1567: F, t3055: F, t1569: F, t2115: F, t1604: F, t2214: F, t3197: F, t514: F, t3177: F, t537: F, t255: F, t571: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9301 = t506 * t8629;
    let t9302 = t529 * t9301;
    let t9311 = t552 * t8629;
    let t9312 = t551 * t9311;
    let t9317 = t1567 * t3055;
    let t9318 = t9317 * t1569;
    let t9319 = t2115 * t9318;
    let t9320 = t1604 * t9319;
    let t9322 = t2214 * t3197;
    let t9323 = t514 * t9322;
    let t9325 = t537 * t3177;
    let t9327 = t571 * t9325 * t255;
    (t9302, t9311, t9312, t9317, t9318, t9319, t9320, t9323, t9327)
}
