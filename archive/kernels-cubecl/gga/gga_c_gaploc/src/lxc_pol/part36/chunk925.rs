//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 925/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk925<F: Float>(t13096: F, t325: F, t550: F, t296: F, t10687: F, t2554: F, t7064: F, t13200: F, t29439: F, t3247: F, t32692: F, t9647: F) -> (F, F, F, F, F, F) {
    let t42920 = t325 * t13096;
    let t42921 = t550 * t42920;
    let t42925 = t296 * t13096;
    let t42931 = t7064 * t10687 * t2554;
    let t42933 = t29439 * t13200;
    let t42934 = F::cast_from(0.1922631557535556071e-2_f64) * t42933;
    let t42936 = t9647 * t32692 * t3247;
    (t42920, t42921, t42925, t42931, t42934, t42936)
}
