//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1293/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1293<F: Float>(t12033: F, t12036: F, t12150: F, t31447: F, t31448: F, t31449: F, t31453: F, t31454: F, t35252: F, t35253: F, t35254: F, t35255: F, t35257: F, t38262: F, t38263: F, t38264: F, t38266: F, t38875: F, t39526: F, t7: F) -> (F,) {
    let t39529 = 2.0 * t12033;
    let t39530 = 4.0 * t12036;
    let tv4rho2sigma24 = -t31447 - t31448 + t31449 - t31453 + t31454 - t38262 + t38263 - t35252 - t38264 + t35253 + t35254 + t35255 + 2.0 * t12150 + t38266 + t7 * (t38875 + t39526) - t39529 - t35257 + t39530;
    (tv4rho2sigma24,)
}
