//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 711/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk711<F: Float>(t11031: F, t11043: F, t11076: F, t11404: F, t11778: F, t11798: F, t16464: F, t16469: F, t16472: F, t16476: F, t8455: F, t16503: F, t16515: F, t16523: F, t103: F, t82: F) -> (F, F) {
    let t16531 = -t11031 - 8.0 / 81.0 * t11043 + t11778 - 8.0 / 27.0 * t11076 - t8455 + t16464 / 6.0 + 4.0 / 27.0 * t11404 - t11798 - t16469 / 12.0 - t16472 / 6.0 + t16476 / 8.0;
    let t16533 = t16503 + t16515 + t16523 + t16531;
    let t16535 = t82 * t16533 * t103;
    (t16533, t16535)
}
