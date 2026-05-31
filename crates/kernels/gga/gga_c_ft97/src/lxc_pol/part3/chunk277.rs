//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 277/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk277<F: Float>(t1053: F, t605: F, t144: F, t1026: F, t1030: F, t1047: F, t28: F, t446: F, t568: F, t89: F, t1045: F, t160: F) -> (F, F, F, F) {
    let t1054 = t605 * t1053;
    let t1055 = t144 * t1054;
    let t1058 = -t568 - t446 * t1026 / F::cast_from(9.0_f64) - t446 * t1030 / F::cast_from(3.0_f64) + t89 * t28 * t1047 / F::cast_from(3.0_f64) - t446 * t1055 / F::cast_from(3.0_f64);
    let t1060 = t1045 * t160;
    (t1054, t1055, t1058, t1060)
}
