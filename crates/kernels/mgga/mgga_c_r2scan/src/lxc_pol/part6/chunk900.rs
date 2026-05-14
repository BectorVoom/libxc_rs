//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 900/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk900<F: Float>(t2150: F, t6398: F, t2147: F, t113: F, t6133: F, t2148: F, t1267: F, t512: F, t57: F) -> (F, F, F, F, F, F) {
    let t6399 = t6398 * t2150;
    let t6400 = t2147 * t6399;
    let t6402 = t6133 * t113;
    let t6403 = t2148 * t6402;
    let t6404 = t2147 * t6403;
    let t6407 = t512 * t1267 * t57;
    (t6399, t6400, t6402, t6403, t6404, t6407)
}
