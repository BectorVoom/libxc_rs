//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1162/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1162<F: Float>(t81095: F, t81102: F, t81105: F, t81124: F, t81131: F, t89051: F, t89054: F, t89058: F, t89062: F, t89069: F, t89073: F, t89077: F, t89081: F, t89085: F) -> F {
    let t89741 = -t89051 / F::cast_from(6.0_f64) + t89054 + F::cast_from(4.0_f64) * t89058 - t89062 / F::cast_from(18.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t81095 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t81102 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t81105 - F::cast_from(6.0_f64) * t89069 + F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t89073 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89077 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89081 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t89085 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t81124 + F::cast_from(20.0_f64) / F::cast_from(243.0_f64) * t81131;
    t89741
}
