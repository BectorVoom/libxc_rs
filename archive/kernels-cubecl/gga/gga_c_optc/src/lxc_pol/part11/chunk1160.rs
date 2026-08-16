//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1160/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1160<F: Float>(t16988: F, t896: F, t7380: F, t51399: F, t935: F, t19: F, t17215: F, t3907: F, t42136: F, t10838: F, t17134: F, t2721: F) -> (F, F, F, F, F) {
    let t52015 = t896 * t16988;
    let t52016 = t52015 * t7380;
    let t52037 = t51399 * t935;
    let t52061 = t52015 * t19;
    let t52111 = t3907 * t42136 * t17215;
    let t52138 = t2721 * t10838 * t17134;
    (t52016, t52037, t52061, t52111, t52138)
}
