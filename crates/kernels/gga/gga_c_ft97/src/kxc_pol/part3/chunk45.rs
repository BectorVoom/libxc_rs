//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 45/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk45<F: Float>(t2: F, t82: F, t24: F, t92: F, t91: F, t85: F) -> (F, F, F, F, F) {
    let t93 = t82 * t2;
    let t94 = t24 * t93;
    let t95 = t92 * t94;
    let t96 = f64::sqrt(t95);
    let t97 = t91 * t96;
    let t100 = F::new(3.0) + t97 / F::new(3.0) + t85 / F::new(3.0);
    (t94, t95, t96, t97, t100)
}
