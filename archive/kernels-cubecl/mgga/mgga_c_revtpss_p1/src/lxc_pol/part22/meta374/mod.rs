//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta374<F: Float>(t13147: F, t460: F, t1269: F, t3555: F, t1275: F, t225: F, t10270: F, t10272: F, t10279: F, t10281: F, t10288: F, t10290: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13148, t13177, t13180, t13181, t13182, t13261, t13262, t13263, t13264, t13265, t13266) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1923::<F>(t13147, t460, t1269, t3555, t1275, t225, t10270, t10272, t10279, t10281, t10288, t10290);
    (t13148, t13177, t13180, t13181, t13182, t13261, t13262, t13263, t13264, t13265, t13266)
}
