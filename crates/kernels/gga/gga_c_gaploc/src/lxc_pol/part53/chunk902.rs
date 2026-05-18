//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 902/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk902<F: Float>(t2508: F, t42944: F, t688: F, t779: F, t2580: F, t28023: F, t2958: F, t3009: F, t7226: F, t23575: F, t3459: F, t11135: F, t7324: F) -> (F, F, F, F, F) {
    let t43325 = F::new(0.76905262301422242837e-2) * t2508 * t779 * t42944 * t688;
    let t43335 = F::new(0.92286314761706691403e-1) * t2508 * t2580 * t2958 * t28023;
    let t43339 = F::new(0.46143157380853345701e-1) * t2508 * t7226 * t3009 * t28023;
    let t43346 = F::new(4.0) * t23575 * t3459;
    let t43353 = F::new(4.0) * t7324 * t11135;
    (t43325, t43335, t43339, t43346, t43353)
}
