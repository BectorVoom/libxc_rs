//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 994/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk994<F: Float>(t2749: F, t5393: F, t296: F, t1255: F, t4129: F, t840: F, t5225: F, t824: F, t10683: F, t319: F, t875: F, t2862: F, t871: F) -> (F, F, F, F, F) {
    let t19391 = t2749 * t5393;
    let t19392 = t296 * t19391;
    let t19396 = t840 * t1255 * t4129;
    let t19399 = t5225 * t824;
    let t19401 = t10683 * t319 * t19399;
    let t19404 = t5225 * t875;
    let t19406 = t2862 * t871 * t19404;
    (t19391, t19392, t19396, t19401, t19406)
}
