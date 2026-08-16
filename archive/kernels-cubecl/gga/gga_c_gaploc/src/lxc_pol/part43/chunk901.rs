//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 901/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk901<F: Float>(t40898: F, t40900: F, t22090: F, t2508: F, t28668: F, t8604: F, t40902: F, t13191: F, t7129: F, t24660: F, t3251: F, t13206: F) -> (F, F, F, F, F, F, F) {
    let t43288 = F::cast_from(0.85450291446024714264e-3_f64) * t40898;
    let t43289 = F::cast_from(0.85450291446024714264e-3_f64) * t40900;
    let t43295 = F::cast_from(0.1845726295234133828e0_f64) * t2508 * t22090 * t8604 * t28668;
    let t43300 = F::cast_from(0.64087718584518535698e-3_f64) * t40902;
    let t43312 = F::cast_from(0.92286314761706691403e-1_f64) * t7129 * t13191;
    let t43315 = F::cast_from(0.92286314761706691403e-1_f64) * t2508 * t24660 * t3251;
    let t43321 = F::cast_from(0.15381052460284448567e-1_f64) * t7129 * t13206;
    (t43288, t43289, t43295, t43300, t43312, t43315, t43321)
}
