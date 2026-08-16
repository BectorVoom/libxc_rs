//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1168/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1168<F: Float>(t43463: F, t43448: F, t43453: F, t43457: F, t43460: F, t43466: F, t43471: F, t43474: F, t43478: F, t43483: F, t43487: F, t43490: F, t43493: F, t43498: F) -> F {
    let t44757 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t43463;
    let t44767 = -t43448 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43453 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43457 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t43460 + t44757 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43466 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t43471 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t43474 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43478 - F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t43483 - t43487 / F::cast_from(18.0_f64) - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t43490 + F::cast_from(20.0_f64) / F::cast_from(243.0_f64) * t43493 + F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t43498;
    t44767
}
