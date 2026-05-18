//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1435/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1435<F: Float>(t12333: F, t31447: F, t31448: F, t31449: F, t31450: F, t31451: F, t31452: F, t31453: F, t31454: F, t31455: F, t35252: F, t35253: F, t35254: F, t35255: F, t35256: F, t35257: F, t35259: F, t39563: F, t39565: F, t7: F) -> F {
    let tv4rho2sigma213 = -t31447 - t31448 + t31449 - t31450 + t31451 + t31452 - t31453 + t31454 - t31455 + t7 * (t39563 + t39565) - t35252 + t35253 + F::new(2.0) * t12333 + t35254 + t35255 - t35256 - t35257 + t35259;
    tv4rho2sigma213
}
