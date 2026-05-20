//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta273<F: Float>(t10565: F, t158: F, t755: F, t9586: F, t2619: F, t2622: F, t2629: F, t9863: F, t123: F, t752: F) -> (F, F, F, F, F) {
        let (t10566, t10568, t10569, t10577, t10578) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1487::<F>(t10565, t158, t755, t9586, t2619, t2622, t2629, t9863, t123, t752);
    (t10566, t10568, t10569, t10577, t10578)
}
