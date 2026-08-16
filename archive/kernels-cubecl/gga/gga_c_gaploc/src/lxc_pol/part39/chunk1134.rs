//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1134/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1134<F: Float>(t13884: F, t2049: F, t47311: F, t739: F, t531: F, t797: F, t43890: F, t43891: F, t43895: F, t43901: F, t47405: F, t47406: F, t47408: F, t47412: F, t47415: F, t47417: F) -> (F, F) {
    let t47419 = F::cast_from(0.35750489951850426669e0_f64) * t2049 * t13884;
    let t47420 = t739 * t47311;
    let t47423 = F::cast_from(0.35750489951850426669e0_f64) * t797 * t531 * t47420;
    let t47425 = t43890 - t43891 + t47405 + t47406 - t47408 - t47412 - t47415 + t47417 - t47419 - t47423 - t43895 + F::cast_from(0.23833659967900284446e0_f64) * t43901;
    (t47420, t47425)
}
