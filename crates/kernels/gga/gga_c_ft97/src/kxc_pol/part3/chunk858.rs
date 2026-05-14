//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 858/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk858<F: Float>(t10683: F, t19399: F, t319: F, t5225: F, t875: F, t2862: F, t871: F, t1212: F, t4129: F, t2749: F, t5330: F, t840: F, t1248: F, t5309: F, t824: F, t2843: F) -> (F, F, F, F, F, F) {
    let t19401 = t10683 * t319 * t19399;
    let t19404 = t5225 * t875;
    let t19406 = t2862 * t871 * t19404;
    let t19409 = t1212 * t4129;
    let t19411 = t2862 * t319 * t19409;
    let t19415 = t840 * t2749 * t5330;
    let t19418 = t4129 * t1248;
    let t19420 = t840 * t871 * t19418;
    let t19423 = t5309 * t824;
    let t19425 = t840 * t2843 * t19423;
    (t19401, t19406, t19411, t19415, t19420, t19425)
}
