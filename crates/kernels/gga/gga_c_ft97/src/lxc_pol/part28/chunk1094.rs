//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1094/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1094<F: Float>(t146143: F, t146171: F, t146182: F, t146208: F, t146218: F, t146340: F, t146473: F, t146552: F, t146593: F, t146937: F, t146972: F, t22907: F, t34352: F, t379: F, t5501: F) -> F {
    let t147004 = -F::cast_from(12.0_f64) * t146937 + F::cast_from(8.0_f64) * t146218 + F::cast_from(8.0_f64) * t146340 + F::cast_from(8.0_f64) * t146182 + F::cast_from(4.0_f64) * t146171 + F::cast_from(4.0_f64) * t146473 + F::cast_from(4.0_f64) * t146552 + F::cast_from(8.0_f64) * t146208 - F::cast_from(12.0_f64) * t146972 + F::cast_from(8.0_f64) * t146143 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5501 * t22907 * t34352 * t379 + F::cast_from(4.0_f64) * t146593;
    t147004
}
