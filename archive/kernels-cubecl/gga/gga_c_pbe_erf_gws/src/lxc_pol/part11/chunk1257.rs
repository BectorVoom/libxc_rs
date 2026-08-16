//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1257/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1257<F: Float>(t11925: F, t12054: F, t11564: F, t45821: F, t3717: F) -> (F, F, F, F) {
    let t49950 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t12054 * t11925;
    let t49952 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t11564 * t11925;
    let t49954 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t45821;
    let t49955 = t3717 * t3717;
    (t49950, t49952, t49954, t49955)
}
