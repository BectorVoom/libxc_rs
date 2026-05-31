//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 276/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk276<F: Float>(t1045: F, t143: F, t160: F, t1000: F, t1020: F, t1041: F, t607: F) -> (F, F) {
    let t1047 = t143 * t1045 * t160;
    let t1053 = t1041 / F::cast_from(2.0_f64) - t607 - t1000 / F::cast_from(3.0_f64) - t1020;
    (t1047, t1053)
}
