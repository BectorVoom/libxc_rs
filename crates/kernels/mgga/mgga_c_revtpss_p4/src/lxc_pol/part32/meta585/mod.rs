//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta585<F: Float>(t102928: F, t25375: F, t1957: F, t28425: F, t25372: F, t98809: F, t25386: F, t95822: F, t98815: F, t95537: F, t25310: F, t28360: F) -> (F, F, F, F, F, F) {
        let (t102930, t102934, t102937, t102939, t102941, t102943) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1914::<F>(t102928, t25375, t1957, t28425, t25372, t98809, t25386, t95822, t98815, t95537, t25310, t28360);
    (t102930, t102934, t102937, t102939, t102941, t102943)
}
