//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1129/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1129<F: Float>(t1603: F, t489: F, t20450: F, t2224: F, t1632: F, t5074: F, t551: F, t574: F, t277: F, t6100: F) -> (F, F, F, F) {
    let t20473 = t1603 * t489;
    let t20475 = t20450 * t20473 * t2224;
    let t20479 = t574 * t551 * t1632 * t5074;
    let t20481 = t6100 * t277;
    (t20473, t20475, t20479, t20481)
}
