//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1231/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1231<F: Float>(t22403: F, t22406: F, t22410: F, t22417: F, t22434: F, t22439: F, t22636: F, t22641: F, t22652: F, t22655: F, t56043: F, t56044: F) -> F {
    let t56262 = -t56043 - t56044 - t22403 - t22636 - t22641 - t22406 - t22410 - t22652 - t22655 - t22417 + t22434 - t22439;
    t56262
}
