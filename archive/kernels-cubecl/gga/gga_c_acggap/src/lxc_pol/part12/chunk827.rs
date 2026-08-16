//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 827/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk827<F: Float>(t7529: F, t7531: F, t7551: F, t7572: F, t7574: F, t7590: F, t7607: F, t8190: F, t8192: F, t8193: F, t8195: F, t8205: F, t8209: F, t8754: F, t8756: F) -> F {
    let t9289 = -t8754 / F::cast_from(24.0_f64) - t8756 / F::cast_from(24.0_f64) - F::cast_from(0.41930789719472202758e-3_f64) * t7529 + F::cast_from(0.94344276868812456207e-3_f64) * t7531 + t8190 + t8192 + t8193 - F::cast_from(0.94344276868812456205e-2_f64) * t7551 - t8195 + t7572 + t7574 - t7590 - t8205 - t7607 + t8209;
    t9289
}
