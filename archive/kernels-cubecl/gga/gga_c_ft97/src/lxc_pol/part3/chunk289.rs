//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 289/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk289<F: Float>(t1131: F, t676: F, t27: F, t89: F, t1089: F, t664: F, t661: F) -> (F, F, F, F) {
    let t1132 = t676 * t1131;
    let t1134 = t89 * t27 * t1132;
    let t1136 = -t664 - t1089 / F::cast_from(18.0_f64) - t1134 / F::cast_from(6.0_f64);
    let t1137 = t661 * t1136;
    (t1132, t1134, t1136, t1137)
}
