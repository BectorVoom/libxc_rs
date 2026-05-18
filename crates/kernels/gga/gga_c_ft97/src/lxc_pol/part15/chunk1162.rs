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
    let t89741 = -t89051 / F::new(6.0) + t89054 + F::new(4.0) * t89058 - t89062 / F::new(18.0) + F::new(4.0) / F::new(9.0) * t81095 - F::new(4.0) / F::new(3.0) * t81102 + F::new(2.0) / F::new(27.0) * t81105 - F::new(6.0) * t89069 + F::new(20.0) / F::new(27.0) * t89073 + F::new(4.0) / F::new(3.0) * t89077 + F::new(4.0) / F::new(3.0) * t89081 - F::new(4.0) / F::new(3.0) * t89085 + F::new(2.0) / F::new(9.0) * t81124 + F::new(20.0) / F::new(243.0) * t81131;
    t89741
}
