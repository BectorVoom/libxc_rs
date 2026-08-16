//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2489/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2489<F: Float>(t2979: F, t43248: F, t50259: F, t50263: F, t62657: F, t62660: F, t62663: F, t62666: F, t62682: F, t62687: F, t68462: F, t68481: F, t973: F) -> F {
    let t70837 = -t62657 / F::cast_from(36.0_f64) + t62660 / F::cast_from(108.0_f64) - t62663 / F::cast_from(144.0_f64) + t62666 / F::cast_from(216.0_f64) + t973 * t2979 * t68481 / F::cast_from(6.0_f64) - t973 * t2979 * t68462 / F::cast_from(12.0_f64) + t50259 - t50263 + t62682 / F::cast_from(1152.0_f64) - t62687 / F::cast_from(576.0_f64) - t43248 / F::cast_from(1944.0_f64);
    t70837
}
