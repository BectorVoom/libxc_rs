//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1425/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1425<F: Float>(t2625: F, t3053: F, t10038: F, t20040: F, t2133: F, t26141: F, t26327: F, t27004: F, t27006: F, t30858: F, t30902: F, t30909: F, t30918: F, t30921: F, t33209: F, t34524: F, t360: F, t6293: F, t7321: F, t8778: F, t8785: F) -> (F, F) {
    let t34528 = t3053 * t2625;
    let t34532 = -0.16463622957338778997e-1 * t30858 - 0.15423020329051080916e-3 * t26327 - 0.15602799132097683414e1 * t26141 * t8785 - 0.51418766733487867048e0 * t27004 - 0.53665922966605306602e-2 * t27006 + 0.13002332610081402845e0 * t2133 * t360 * t8778 * t33209 - 0.26004665220162805689e0 * t20040 * t10038 + 0.83214928704520978208e1 * t30902 - 0.10401866088065122276e1 * t30909 + 0.1047928639570397803e0 * t30918 + 0.1047928639570397803e0 * t30921 - 0.49390868872016336989e0 * t6293 * t7321 * t34524 - 0.49390868872016336988e0 * t6293 * t7321 * t34528;
    (t34528, t34532)
}
