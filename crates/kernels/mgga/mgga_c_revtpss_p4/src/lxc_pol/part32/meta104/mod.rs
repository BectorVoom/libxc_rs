//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk618;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta104<F: Float>(t177: F, t752: F, t762: F, t717: F, t750: F, t675: F, t723: F, t169: F, t722: F, t164: F, t729: F, t730: F) -> (F, F, F, F, F, F, F, F) {
        let (t2523, t2524, t2526, t2531, t2536, t2537, t2538, t2539) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk618::<F>(t177, t752, t762, t717, t750, t675, t723, t169, t722, t164, t729, t730);
    (t2523, t2524, t2526, t2531, t2536, t2537, t2538, t2539)
}
