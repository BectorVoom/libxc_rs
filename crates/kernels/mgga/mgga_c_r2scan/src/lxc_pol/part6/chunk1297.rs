//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1297/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1297<F: Float>(t2133: F, t2294: F, t7442: F, t7494: F, t8014: F, t2115: F, t6188: F, t6189: F, t1569: F, t2590: F, t494: F, t5: F, t7: F, t8029: F, t8030: F, t2106: F, t2834: F) -> (F, F, F, F, F, F) {
    let t24433 = t2133 * t2294 * t7442;
    let t24439 = t7494 * t8014;
    let t24442 = t6188 * t6189 * t2115;
    let t24447 = t24442 * t2590 * t1569 * t5 * t7 * t494;
    let t24450 = t8029 * t2294 * t8030;
    let t24452 = t2834 * t2106;
    (t24433, t24439, t24442, t24447, t24450, t24452)
}
