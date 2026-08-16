//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1216;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1217;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta314<F: Float>(t755: F, t9586: F, t2619: F, t2622: F, t2390: F, t72: F, t757: F, t2629: F, t9863: F, t123: F, t752: F, t2630: F, t9866: F, t9575: F, t9572: F, t177: F, t762: F, t760: F, t9419: F, t2516: F, t2523: F, t9387: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10568, t10569, t10574, t10577, t10579) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1216::<F>(t755, t9586, t2619, t2622, t2390, t72, t757, t2629, t9863, t123, t752, t2630);
        let (t10582, t10584, t10586, t10588, t10592, t10593, t10596) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1217::<F>(t2629, t9866, t9575, t9572, t177, t2390, t762, t760, t9419, t2516, t2523, t9387);
    (t10568, t10569, t10574, t10577, t10579, t10582, t10584, t10586, t10588, t10592, t10593, t10596)
}
