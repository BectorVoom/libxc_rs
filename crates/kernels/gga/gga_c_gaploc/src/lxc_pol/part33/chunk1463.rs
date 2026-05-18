//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1463/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1463<F: Float>(t12339: F, t31447: F, t31448: F, t31449: F, t31453: F, t31454: F, t35252: F, t35253: F, t35254: F, t35255: F, t35257: F, t38262: F, t38263: F, t38264: F, t38266: F, t39529: F, t39530: F, t39577: F, t39579: F, t7: F) -> F {
    let tv4rho2sigma216 = -t31447 - t31448 + t31449 - t31453 + t31454 - t38262 + t38263 + t7 * (t39577 + t39579) - t35252 - t38264 + t35253 + t35254 + t35255 + F::new(2.0) * t12339 + t38266 - t39529 - t35257 + t39530;
    tv4rho2sigma216
}
