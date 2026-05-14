//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1125/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1125<F: Float>(t2201: F, t6566: F, t785: F, t788: F, t277: F, t5073: F, t1569: F, t2148: F, t6535: F, t113: F, t20001: F, t2115: F, t1604: F, t1584: F, t6385: F, t108: F, t2214: F) -> (F, F, F, F, F, F, F) {
    let t20371 = t2201 * t785 * t788 * t6566;
    let t20373 = t277 * t5073;
    let t20374 = t20373 * t1569;
    let t20376 = t6535 * t2148 * t20374;
    let t20378 = t20001 * t113;
    let t20379 = t2115 * t20378;
    let t20380 = t1604 * t20379;
    let t20384 = t1584 * t6385;
    let t20407 = t2214 * t108;
    (t20371, t20373, t20376, t20379, t20380, t20384, t20407)
}
