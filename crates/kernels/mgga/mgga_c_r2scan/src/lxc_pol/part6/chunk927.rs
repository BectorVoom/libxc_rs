//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 927/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk927<F: Float>(t5166: F, t6112: F, t6187: F, t6274: F, t6369: F, t6433: F, t6498: F, t6589: F, t1551: F, t1554: F, t1556: F, t1562: F, t2259: F, t2534: F, t285: F, t495: F, t499: F, t5066: F, t5068: F, t5074: F, t5078: F, t5081: F, t5087: F, t5088: F, t792: F) -> (F, F) {
    let t6592 = t5166 + t6112 + t6187 + t6274 + t6369 + t6433 + t6498 + t6589;
    let t6595 = t5066 * t285 + 3.0 * t5068 * t2534 + 3.0 / 4.0 * t1551 * t1556 + t5074 * t285 + 3.0 / 4.0 * t1554 * t1556 - 15.0 / 16.0 * t495 * t5078 + 3.0 / 4.0 * t495 * t5081 + 45.0 / 64.0 * t5087 * t5088 - 15.0 / 16.0 * t1562 * t792 * t2259 + t499 * t6592 / 4.0;
    (t6592, t6595)
}
