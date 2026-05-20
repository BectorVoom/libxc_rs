//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2618;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta744<F: Float>(t13581: F, t177: F, t762: F, t46971: F, t1317: F, t13632: F, t3857: F, t5569: F, t512: F, t749: F, t46973: F, t3863: F, t5567: F) -> (F, F, F, F, F, F, F) {
        let (t48223, t48224, t48226, t48228, t48231, t48232, t48234) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2618::<F>(t13581, t177, t762, t46971, t1317, t13632, t3857, t5569, t512, t749, t46973, t3863, t5567);
    (t48223, t48224, t48226, t48228, t48231, t48232, t48234)
}
