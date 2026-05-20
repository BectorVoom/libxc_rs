//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk509;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk510;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk511;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta86<F: Float>(t177: F, t752: F, t762: F, t717: F, t750: F, t675: F, t723: F, t169: F, t722: F, t164: F, t729: F, t730: F, t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2523 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk509::<F>(t177, t752);
        let (t2525, t2527, t2531, t2535, t2536, t2537, t2538) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk510::<F>(t2523, t762, t717, t750, t675, t723, t169, t722, t164, t729);
        let (t2539, t2548) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk511::<F>(t2538, t730, t2435, t2439, t2502, t2504, t2509, t2511);
        let (t2549, t2552) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk512::<F>(t2548, t730, t722);
    (t2523, t2525, t2527, t2531, t2535, t2536, t2537, t2538, t2539, t2548, t2549, t2552)
}
