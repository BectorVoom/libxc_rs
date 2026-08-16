//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1895;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta458<F: Float>(t19634: F, t6271: F, t3117: F, t12131: F, t357: F, t4786: F, t6100: F, t3092: F, t1065: F, t6244: F, t906: F, t1042: F, t3172: F, t6301: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19635, t19636, t19639) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1895::<F>(t19634, t6271, t3117, t12131, t357);
        let (t19640, t19641, t19644, t19645, t19649, t19650, t19651, t19658) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1896::<F>(t19639, t6271, t3117, t4786, t6100, t3092, t1065, t6244, t906, t1042, t3172, t6301);
    (t19635, t19636, t19639, t19640, t19641, t19644, t19645, t19649, t19650, t19651, t19658)
}
