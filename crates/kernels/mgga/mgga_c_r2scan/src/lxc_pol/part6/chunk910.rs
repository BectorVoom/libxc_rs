//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 910/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk910<F: Float>(t2191: F, t2236: F, t1632: F, t2252: F, t551: F, t549: F, t2097: F, t547: F) -> (F, F, F, F) {
    let t6468 = t2236 * t2191;
    let t6470 = t1632 * t2252;
    let t6471 = t551 * t6470;
    let t6472 = t549 * t6471;
    let t6474 = t547 * t2097;
    (t6468, t6471, t6472, t6474)
}
