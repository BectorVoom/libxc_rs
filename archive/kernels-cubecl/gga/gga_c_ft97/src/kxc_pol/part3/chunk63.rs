//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 63/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk63<F: Float>(t143: F, t2: F, t24: F, t92: F, t91: F, t146: F) -> (F, F, F, F, F) {
    let t150 = t143 * t2;
    let t151 = t24 * t150;
    let t152 = t92 * t151;
    let t153 = F::sqrt(t152);
    let t154 = t91 * t153;
    let t157 = F::cast_from(3.0_f64) + t154 / F::cast_from(3.0_f64) + t146 / F::cast_from(3.0_f64);
    (t151, t152, t153, t154, t157)
}
