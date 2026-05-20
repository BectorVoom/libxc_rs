//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk611;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk612;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta95<F: Float>(t2014: F, t2035: F, t118: F, t1932: F, t1939: F, t2007: F, t2011: F, t508: F, t569: F, t3: F, param_d: F, t117: F, t1936: F, t572: F, t573: F, t10: F, t17: F, t576: F, t580: F, t15: F, t22: F, t11: F, t14: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2037, t2038, t2040) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk611::<F>(t2014, t2035, t118, t1932, t1939, t2007, t2011, t508, t569, t3, param_d);
        let t2042 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk612::<F>(t117, t1936);
        let (t2045, t2219, t2221, t2223, t2224) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk613::<F>(t2042, t572, t2040, t573, t10, t17, t576, t580, t15, t22, t11, t14);
    (t2037, t2038, t2040, t2042, t2045, t2219, t2221, t2223, t2224)
}
