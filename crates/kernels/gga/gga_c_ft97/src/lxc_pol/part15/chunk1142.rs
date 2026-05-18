//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1142/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1142<F: Float>(t446: F, t89083: F, t9770: F, t81095: F, t81102: F, t81105: F, t81124: F, t81131: F, t89047: F, t89051: F, t89054: F, t89058: F, t89062: F, t89069: F, t89073: F, t89077: F, t89081: F) -> (F, F) {
    let t89085 = t446 * t9770 * t89083;
    let t89089 = -F::new(80.0) / F::new(81.0) * t89047 - t89051 + F::new(6.0) * t89054 + F::new(24.0) * t89058 - t89062 / F::new(3.0) + F::new(8.0) / F::new(3.0) * t81095 - F::new(8.0) * t81102 + F::new(4.0) / F::new(9.0) * t81105 - F::new(36.0) * t89069 + F::new(40.0) / F::new(9.0) * t89073 + F::new(8.0) * t89077 + F::new(8.0) * t89081 - F::new(8.0) * t89085 + F::new(4.0) / F::new(3.0) * t81124 + F::new(40.0) / F::new(81.0) * t81131;
    (t89085, t89089)
}
