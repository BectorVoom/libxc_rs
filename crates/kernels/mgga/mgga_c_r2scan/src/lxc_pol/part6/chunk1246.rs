//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1246/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1246<F: Float>(t7094: F, t7096: F, t6786: F, t6907: F, t7042: F, t7045: F, t7057: F, t7090: F, t7092: F, t8517: F, t9914: F, t9915: F, t9916: F, t7125: F, t5024: F, t5026: F, t6884: F, t6893: F, t7116: F, t7120: F, t7141: F, t7143: F, t7147: F, t9917: F, t9918: F, t9921: F) -> (F, F) {
    let t23306 = 24.0 * t7094;
    let t23307 = 24.0 * t7096;
    let t23308 = -6.0 * t7042 + 3.0 * t8517 + 18.0 * t7045 - t9914 + t9915 + t6786 + t6907 + 36.0 * t7057 + 9.0 * t7090 + 0.59255020495841404221e-1 * t7092 + t23306 - t23307 - t9916;
    let t23312 = 6.0 * t7125;
    let t23316 = t9917 + t9918 - 9.0 * t7116 + t5024 + 18.0 * t7120 + t6884 + t23312 + t9921 - 3.0 * t7141 + 18.0 * t7143 - 18.0 * t7147 - t6893 + t5026;
    (t23308, t23316)
}
