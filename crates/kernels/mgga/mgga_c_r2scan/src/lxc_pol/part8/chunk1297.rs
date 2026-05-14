//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1297/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1297<F: Float>(t6518: F, t9131: F, t2195: F, t9463: F, t2183: F, t3053: F, t481: F, t538: F, t6155: F, t2294: F, t7461: F, t8807: F, t2139: F, t9143: F, t29775: F, t6086: F, t6093: F) -> (F, F, F, F, F, F, F, F) {
    let t30844 = t6518 * t9131;
    let t30850 = t2195 * t9463;
    let t30853 = t2183 * t9463;
    let t30856 = t3053 * t481;
    let t30858 = t6155 * t538 * t30856;
    let t30902 = t7461 * t2294 * t8807;
    let t30909 = t2139 * t2294 * t9143;
    let t30918 = t6093 * t6086 * t29775;
    (t30844, t30850, t30853, t30856, t30858, t30902, t30909, t30918)
}
