//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1252/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1252<F: Float>(t2294: F, t8029: F, t9543: F, t6139: F, t9501: F, t2133: F, t9169: F, t7365: F, t7987: F, t9165: F, t2139: F, t8833: F, t8176: F, t8240: F, t1632: F, t2196: F, t551: F, t9115: F) -> (F, F, F, F, F, F, F, F) {
    let t28258 = t8029 * t2294 * t9543;
    let t28261 = t6139 * t2294 * t9501;
    let t28264 = t2133 * t2294 * t9169;
    let t28266 = t7987 * t7365;
    let t28273 = t2133 * t2294 * t9165;
    let t28276 = t2139 * t2294 * t8833;
    let t28292 = t8240 * t8176;
    let t28296 = t2196 * t551 * t1632 * t9115;
    (t28258, t28261, t28264, t28266, t28273, t28276, t28292, t28296)
}
