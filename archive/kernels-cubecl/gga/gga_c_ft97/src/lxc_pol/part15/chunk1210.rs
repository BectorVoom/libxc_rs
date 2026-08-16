//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1210/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1210<F: Float>(t83718: F, t83720: F, t89859: F, t89865: F, t89868: F, t89872: F, t89875: F, t89879: F, t89883: F, t89887: F, t89891: F, t90307: F, t90311: F, t90316: F) -> F {
    let t91186 = -F::cast_from(6.0_f64) * t89859 + F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t89865 - F::cast_from(2.0_f64) * t89868 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t89872 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t89875 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t89879 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t89883 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t89887 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t89891 - t90307 / F::cast_from(6.0_f64) + F::cast_from(4.0_f64) * t90311 + t90316 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t83718 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t83720;
    t91186
}
