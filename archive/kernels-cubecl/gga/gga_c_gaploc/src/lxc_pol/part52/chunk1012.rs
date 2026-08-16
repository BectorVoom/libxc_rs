//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 1012/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk1012<F: Float>(t13334: F, t13338: F, t13342: F, t13345: F, t13349: F, t13352: F, t14290: F, t14292: F, t14294: F, t14297: F, t14349: F, t14444: F, t51228: F, t51229: F, t7: F) -> F {
    let tv4rhosigma317 = t14290 - t14292 + t13342 - t13349 - t13334 + t13338 - t14294 - t13345 + t13352 + t14297 - t14349 + t14444 + t7 * (t51228 + t51229);
    tv4rhosigma317
}
