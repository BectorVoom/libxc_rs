//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 975/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk975<F: Float>(t13880: F, t784: F, t13884: F, t2049: F, t47311: F, t739: F, t531: F, t797: F, t13879: F, t2009: F, t773: F, t38950: F, t955: F) -> (F, F, F, F, F, F) {
    let t47417 = F::new(0.23833659967900284446e0) * t13880 * t784;
    let t47419 = F::new(0.35750489951850426669e0) * t2049 * t13884;
    let t47420 = t739 * t47311;
    let t47423 = F::new(0.35750489951850426669e0) * t797 * t531 * t47420;
    let t47430 = F::new(0.35750489951850426669e0) * t773 * t13879 * t2009;
    let t47432 = t955 * t38950;
    (t47417, t47419, t47420, t47423, t47430, t47432)
}
