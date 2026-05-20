//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk714;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta103<F: Float>(t2523: F, t762: F, t717: F, t750: F, t675: F, t723: F, t169: F, t722: F, t164: F, t729: F, t730: F, t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2524, t2525, t2526, t2527, t2531, t2535, t2536, t2537, t2538) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk714::<F>(t2523, t762, t717, t750, t675, t723, t169, t722, t164, t729);
        let (t2539, t2548) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk715::<F>(t2538, t730, t2435, t2439, t2502, t2504, t2509, t2511);
    (t2524, t2525, t2526, t2527, t2531, t2535, t2536, t2537, t2538, t2539, t2548)
}
