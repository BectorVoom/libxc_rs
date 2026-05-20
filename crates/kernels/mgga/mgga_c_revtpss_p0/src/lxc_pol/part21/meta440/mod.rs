//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta440<F: Float>(t2470: F, t5721: F, t3915: F, t1445: F, t5599: F, t689: F, t2435: F, t5600: F, t1426: F, t1893: F, t786: F, t3917: F) -> (F, F, F, F, F, F, F, F) {
        let (t14090, t14091, t14094, t14096, t14097, t14099, t14100, t14102) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1956::<F>(t2470, t5721, t3915, t1445, t5599, t689, t2435, t5600, t1426, t1893, t786, t3917);
    (t14090, t14091, t14094, t14096, t14097, t14099, t14100, t14102)
}
