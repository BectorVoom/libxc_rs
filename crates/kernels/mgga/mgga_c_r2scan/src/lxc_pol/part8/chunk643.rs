//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 643/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk643<F: Float>(t1556: F, t1562: F, t2532: F, t2533: F, t2534: F, t2538: F, t2541: F, t2847: F, t495: F, t499: F, t921: F) -> (F,) {
    let t2850 = t2532 + t2533 * t2534 + t921 * t1556 / 4.0 + t495 * t2538 / 4.0 - 5.0 / 16.0 * t1562 * t2541 + t499 * t2847 / 4.0;
    (t2850,)
}
