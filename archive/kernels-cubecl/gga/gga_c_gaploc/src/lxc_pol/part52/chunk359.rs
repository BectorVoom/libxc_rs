//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 359/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk359<F: Float>(t2963: F, t738: F, t1025: F, t1030: F, t1841: F, t1897: F, t2508: F, t2545: F, t2550: F, t2556: F, t2560: F, t2565: F, t270: F, t2909: F, t2912: F, t2928: F, t2933: F, t2937: F, t2951: F, t2955: F, t2960: F, t650: F, t681: F) -> F {
    let t2964 = t738 * t2963;
    let t2967 = F::cast_from(0.10254034973522965712e-1_f64) * t650 * t1025 + F::cast_from(0.76905262301422242837e-2_f64) * t681 * t1025 - F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t2909 + F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2912 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t2928 - F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t2933 - F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t2937 - F::cast_from(0.85450291446024714264e-3_f64) * t2545 + F::cast_from(0.64087718584518535698e-3_f64) * t2550 - F::cast_from(0.64087718584518535698e-3_f64) * t2556 + F::cast_from(0.64087718584518535698e-3_f64) * t2560 - F::cast_from(0.64087718584518535698e-3_f64) * t2565 - F::cast_from(0.10254034973522965712e-1_f64) * t650 * t1030 - F::cast_from(0.76905262301422242837e-2_f64) * t681 * t1030 + F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t2951 + F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t2955 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2960 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t2964;
    t2967
}
