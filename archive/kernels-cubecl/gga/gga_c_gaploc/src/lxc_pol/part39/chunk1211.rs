//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1211/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1211<F: Float>(t40546: F, t40564: F, t42452: F, t42455: F, t42456: F, t42457: F, t42458: F, t42460: F, t42461: F, t48217: F, t48221: F, t48225: F, t48231: F) -> F {
    let t48233 = F::cast_from(0.38342925953920749677e0_f64) * t40546;
    let t48235 = -F::cast_from(0.61348681526273199483e1_f64) * t48217 - F::cast_from(0.46011511144704899612e1_f64) * t48221 - F::cast_from(0.46011511144704899612e1_f64) * t48225 - t48231 - F::cast_from(0.25025342966295298669e1_f64) * t42452 + t48233 + t42455 - t42456 + t42457 - t42458 + F::cast_from(0.10224780254378866581e1_f64) * t40564 - t42460 + t42461;
    t48235
}
