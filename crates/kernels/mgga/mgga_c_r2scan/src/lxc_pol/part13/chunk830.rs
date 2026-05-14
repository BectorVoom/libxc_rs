//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 830/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk830<F: Float>(t1551: F, t1554: F, t1556: F, t1562: F, t1563: F, t2259: F, t2531: F, t2533: F, t2534: F, t2538: F, t2847: F, t285: F, t495: F, t499: F, t5078: F, t5081: F, t5087: F, t7195: F, t7197: F, t7202: F, t7204: F, t7206: F, t7218: F, t7221: F, t792: F, t8296: F, t921: F, t983: F) -> (F,) {
    let t8299 = t7195 * t285 + 2.0 * t7197 * t2534 + t2531 * t1556 / 2.0 + t7202 * t2534 + t7204 * t2534 + t2533 * t7206 / 2.0 - 5.0 / 16.0 * t921 * t5078 + t921 * t5081 / 4.0 + t1551 * t2538 / 4.0 + t1554 * t2538 / 4.0 - 5.0 / 8.0 * t495 * t7218 + t495 * t7221 / 2.0 + 45.0 / 64.0 * t5087 * t983 * t1563 - 5.0 / 8.0 * t1562 * t2847 * t792 - 5.0 / 16.0 * t1562 * t983 * t2259 + t499 * t8296 / 4.0;
    (t8299,)
}
