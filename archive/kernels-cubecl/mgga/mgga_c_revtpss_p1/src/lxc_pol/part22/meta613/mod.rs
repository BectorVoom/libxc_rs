//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta613<F: Float>(t19620: F, t6271: F, t3117: F, t19501: F, t3095: F, t3092: F, t1043: F, t3155: F, t12131: F, t357: F, t4786: F, t6100: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19621, t19622, t19625, t19626, t19634, t19635, t19636, t19639, t19640, t19641, t19644) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2518::<F>(t19620, t6271, t3117, t19501, t3095, t3092, t1043, t3155, t12131, t357, t4786, t6100);
    (t19621, t19622, t19625, t19626, t19634, t19635, t19636, t19639, t19640, t19641, t19644)
}
