//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 829/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk829<F: Float>(t8829: F, t7649: F, t7651: F, t7653: F, t7655: F, t8232: F, t8801: F, t8804: F, t8808: F, t8811: F, t8814: F, t8818: F, t8821: F, t8824: F, t8827: F) -> F {
    let t9309 = F::cast_from(0.84046875e-1_f64) * t8829;
    let t9310 = t7649 + t8801 / F::cast_from(64.0_f64) + t8804 / F::cast_from(96.0_f64) + t8808 / F::cast_from(8.0_f64) + t8811 / F::cast_from(24.0_f64) + F::cast_from(0.22921875e-1_f64) * t8814 + F::cast_from(0.22921875e-1_f64) * t8818 + F::cast_from(0.1528125e-1_f64) * t8821 + F::cast_from(0.22921875e-1_f64) * t8824 + F::cast_from(0.1528125e-1_f64) * t8827 - t9309 + t7651 - t7653 + t7655 + t8232;
    t9310
}
