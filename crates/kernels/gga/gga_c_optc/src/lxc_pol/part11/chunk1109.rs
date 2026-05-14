//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1109/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1109<F: Float>(t22296: F, t22300: F, t22344: F, t22621: F, t22623: F, t22625: F, t22627: F, t56013: F, t56014: F, t56015: F, t56016: F, t56039: F, t56040: F, t22403: F, t22406: F, t22410: F, t22417: F, t22434: F, t22439: F, t22636: F, t22641: F, t22652: F, t22655: F, t56043: F, t56044: F) -> (F, F) {
    let t56259 = -t56013 + t56014 + t56015 - t56016 + t22296 - t22300 + t22344 + t22621 - t22623 + t22625 + t22627 - t56039 - t56040;
    let t56262 = -t56043 - t56044 - t22403 - t22636 - t22641 - t22406 - t22410 - t22652 - t22655 - t22417 + t22434 - t22439;
    (t56259, t56262)
}
