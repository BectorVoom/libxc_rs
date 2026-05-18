//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 735/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk735<F: Float>(t10478: F, t319: F, t2766: F, t871: F, t10491: F, t2843: F, t848: F, t1221: F, t8232: F, t1242: F, t2399: F, t89: F) -> (F, F, F, F, F, F) {
    let t15290 = t10478 * t319;
    let t15294 = t2766 * t871;
    let t15299 = t10491 * t319;
    let t15312 = t848 * t2843;
    let t15318 = t8232 * t1221;
    let t15329 = t89 * t2399 * t1242;
    (t15290, t15294, t15299, t15312, t15318, t15329)
}
