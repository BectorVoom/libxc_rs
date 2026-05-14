//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1321/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1321<F: Float>(t1592: F, t1632: F, t551: F, t8235: F, t6518: F, t7593: F, t20407: F, t2552: F, t5147: F, t1234: F, t2526: F, t2531: F, t574: F, t6343: F, t2184: F, t2634: F) -> (F, F, F, F, F, F) {
    let t24943 = t1592 * t551 * t1632 * t8235;
    let t24945 = t6518 * t7593;
    let t24948 = t5147 * t20407 * t2552;
    let t24955 = t2526 * t1234;
    let t24962 = t574 * t551 * t6343 * t2531;
    let t24963 = 0.12713391885412927226e1 * t24962;
    let t24966 = t2184 * t551 * t6343 * t2634;
    (t24943, t24945, t24948, t24955, t24963, t24966)
}
