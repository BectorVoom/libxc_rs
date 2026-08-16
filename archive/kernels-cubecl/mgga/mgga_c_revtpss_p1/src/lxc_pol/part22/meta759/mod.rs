//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta759 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta759<F: Float>(t1041: F, t1046: F, t42994: F, t1086: F, t11213: F, t3090: F, t3057: F, t3316: F, t4891: F, t3298: F, t3059: F, t3154: F) -> (F, F, F, F, F, F, F) {
        let (t42996, t43038, t43043, t43044, t43049, t43050, t43051) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2839::<F>(t1041, t1046, t42994, t1086, t11213, t3090, t3057, t3316, t4891, t3298, t3059, t3154);
    (t42996, t43038, t43043, t43044, t43049, t43050, t43051)
}
