//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 857/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk857<F: Float>(t5393: F, t824: F, t840: F, t871: F, t875: F, t2843: F, t296: F, t15128: F, t4181: F, t1882: F, t5419: F, t5381: F, t2749: F, t1255: F, t4129: F, t5225: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19373 = t5393 * t824;
    let t19375 = t840 * t871 * t19373;
    let t19378 = t5393 * t875;
    let t19379 = t2843 * t19378;
    let t19380 = t296 * t19379;
    let t19383 = t15128 * t4181;
    let t19384 = t296 * t19383;
    let t19387 = t1882 * t5419;
    let t19389 = t1882 * t5381;
    let t19391 = t2749 * t5393;
    let t19392 = t296 * t19391;
    let t19396 = t840 * t1255 * t4129;
    let t19399 = t5225 * t824;
    (t19375, t19379, t19380, t19383, t19384, t19387, t19389, t19391, t19392, t19396, t19399)
}
