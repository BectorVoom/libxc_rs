//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 941/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk941<F: Float>(t30596: F, t30607: F, t30611: F, t34482: F, t34484: F, t34485: F, t34489: F, t34492: F, t34497: F, t34499: F, t34501: F, t34502: F, t34504: F, t34507: F, t34508: F, t34510: F, t34513: F, t34516: F) -> (F,) {
    let t34518 = -0.25724410870841842183e-2 * t34482 + t30596 - t34484 - t34485 + 0.140078125e-1 * t30607 + t34489 - 0.15724046144802076034e-3 * t34492 - 0.25724410870841842184e-2 * t30611 + 0.62896184579208304136e-3 * t34497 - t34499 + t34501 - 0.17149607247227894789e-2 * t34502 - 0.85748036236139473944e-3 * t34504 + t34507 - 0.15724046144802076034e-2 * t34508 + 0.66040993808168719343e-2 * t34510 - t34513 + 0.20965394859736101379e-2 * t34516;
    (t34518,)
}
