//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1029 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1029<F: Float>(t3391: F, t43821: F, t6442: F, t12327: F, t6449: F, t43946: F, t12331: F, t16926: F, t5071: F, t1134: F, t20337: F, t3390: F) -> (F, F, F, F, F, F) {
        let (t68470, t68473, t68476, t68479, t68481, t68484) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3612::<F>(t3391, t43821, t6442, t12327, t6449, t43946, t12331, t16926, t5071, t1134, t20337, t3390);
    (t68470, t68473, t68476, t68479, t68481, t68484)
}
