//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1436/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1436<F: Float>(t106: F, t34771: F, t34815: F, t797: F, t97: F, t2625: F, t2858: F, t8597: F, t10572: F, t6897: F, t2266: F, t481: F, t10392: F, t2333: F, t1048: F, t795: F) -> (F, F, F, F) {
    let t34819 = t97 * t106 * (t34771 + t34815) * t797;
    let t34822 = 18.0 * t2858 * t8597 * t2625;
    let t34824 = t10572 * t6897;
    let t34827 = 6.0 * t2266 * t34824 * t481;
    let t34828 = t10392 * t2333;
    let t34830 = t1048 * t34828 * t795;
    (t34819, t34822, t34827, t34830)
}
