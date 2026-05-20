//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta635<F: Float>(t3531: F, t6556: F, t6552: F, t3362: F, t5825: F, t606: F, t3417: F, t141: F, t1121: F, t18281: F) -> (F, F, F, F, F, F, F) {
        let (t20261, t20263, t20265, t20266, t20267, t20268, t20272) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2562::<F>(t3531, t6556, t6552, t3362, t5825, t606, t3417, t141, t1121, t18281);
    (t20261, t20263, t20265, t20266, t20267, t20268, t20272)
}
