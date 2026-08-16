//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk584;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk585;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk586;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta94<F: Float>(t1450: F, t2107: F, t118: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t508: F, t569: F, t651: F, t3: F, param_d: F, t117: F, t2055: F, t572: F, t573: F, t10: F, t17: F, t576: F, t580: F, t15: F, t22: F, t11: F, t14: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t2108 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk584::<F>(t1450, t2107);
        let (t2110, t2111, t2113) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk585::<F>(t118, t2014, t2052, t2056, t2089, t2093, t2108, t508, t569, t651, t3, param_d);
        let t2115 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk586::<F>(t117, t2055);
        let (t2118, t2219, t2221, t2223, t2224) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk587::<F>(t2113, t2115, t572, t573, t10, t17, t576, t580, t15, t22, t11, t14);
    (t2108, t2110, t2111, t2113, t2115, t2118, t2219, t2221, t2223, t2224)
}
