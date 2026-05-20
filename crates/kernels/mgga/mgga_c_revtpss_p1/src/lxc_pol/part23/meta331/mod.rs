//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1630;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta331<F: Float>(t1364: F, t14082: F, t1357: F, t5775: F, t689: F, t2470: F, t5721: F, t3915: F, t1445: F, t5599: F, t2435: F, t5600: F, t1426: F, t1893: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14084, t14085, t14087, t14090, t14091, t14094, t14096, t14097) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1630::<F>(t1364, t14082, t1357, t5775, t689, t2470, t5721, t3915, t1445, t5599, t2435, t5600);
        let (t14099, t14100) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1631::<F>(t1426, t1893, t786);
    (t14084, t14085, t14087, t14090, t14091, t14094, t14096, t14097, t14099, t14100)
}
