//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1153/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1153<F: Float>(t147730: F, t5899: F, t95344: F, t34943: F, t375: F, t89: F, t147590: F, t27: F, t526: F, t139453: F, t139485: F, t139493: F, t139496: F, t148593: F, t148597: F, t148601: F, t148604: F, t148607: F, t148611: F, t148616: F, t148621: F, t148625: F) -> (F, F, F, F) {
    let t148629 = t5899 * t95344 * t147730;
    let t148632 = t89 * t375 * t34943;
    let t148636 = t89 * t27 * t526 * t147590;
    let t148638 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t148593 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t148597 - F::cast_from(2.0_f64) * t148601 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148604 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t148607 + t139453 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t148611 + t148616 / F::cast_from(3.0_f64) + t148621 / F::cast_from(12.0_f64) - F::cast_from(4.0_f64) * t148625 + t139485 / F::cast_from(27.0_f64) + t148629 / F::cast_from(3.0_f64) + t148632 / F::cast_from(9.0_f64) - t148636 / F::cast_from(3.0_f64) - t139493 + t139496;
    (t148629, t148632, t148636, t148638)
}
