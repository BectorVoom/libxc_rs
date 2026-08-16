//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1231/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1231(t22403: f64, t22406: f64, t22410: f64, t22417: f64, t22434: f64, t22439: f64, t22636: f64, t22641: f64, t22652: f64, t22655: f64, t56043: f64, t56044: f64) -> f64 {
    let t56262 = -t56043 - t56044 - t22403 - t22636 - t22641 - t22406 - t22410 - t22652 - t22655 - t22417 + t22434 - t22439;
    t56262
}
