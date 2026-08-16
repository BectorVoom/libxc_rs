//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 889/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk889(t32357: f64, t5539: f64, t9647: f64, t32436: f64, t13212: f64, t7137: f64, t13209: f64, t7129: f64, t2508: f64, t3255: f64, t8637: f64, t2936: f64, t9689: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42988 = t9647 * t5539 * t32357;
    let t42991 = t9647 * t5539 * t32436;
    let t42998 = 0.30762104920568897135e-1_f64 * t7137 * t13212;
    let t43006 = 0.76905262301422242837e-2_f64 * t7129 * t13209;
    let t43014 = 0.23071578690426672851e-1_f64 * t2508 * t8637 * t3255;
    let t43017 = 0.23071578690426672851e-1_f64 * t2508 * t2936 * t9689;
    (t42988, t42991, t42998, t43006, t43014, t43017)
}
