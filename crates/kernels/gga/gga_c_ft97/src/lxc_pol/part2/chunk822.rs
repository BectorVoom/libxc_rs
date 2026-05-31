//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 822/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk822<F: Float>(t12362: F, t12365: F, t12353: F, t12359: F, t12564: F, t12568: F, t12911: F, t8799: F, t8802: F, t9059: F, t9383: F, t12571: F) -> (F, F) {
    let t12913 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t12362;
    let t12914 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12365;
    let t12917 = t8799 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t8802 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9059 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12353 - t12911 + F::cast_from(22.0_f64) / F::cast_from(27.0_f64) * t12359 - t12913 - t9383 + t12914 - t12564 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12568;
    let t12918 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12571;
    (t12917, t12918)
}
