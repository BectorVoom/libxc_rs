//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 889/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk889<F: Float>(t32357: F, t5539: F, t9647: F, t32436: F, t13212: F, t7137: F, t13209: F, t7129: F, t2508: F, t3255: F, t8637: F, t2936: F, t9689: F) -> (F, F, F, F, F, F) {
    let t42988 = t9647 * t5539 * t32357;
    let t42991 = t9647 * t5539 * t32436;
    let t42998 = F::cast_from(0.30762104920568897135e-1_f64) * t7137 * t13212;
    let t43006 = F::cast_from(0.76905262301422242837e-2_f64) * t7129 * t13209;
    let t43014 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t8637 * t3255;
    let t43017 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t2936 * t9689;
    (t42988, t42991, t42998, t43006, t43014, t43017)
}
