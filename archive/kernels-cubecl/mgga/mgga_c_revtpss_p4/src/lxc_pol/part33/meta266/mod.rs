//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1185;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1186;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1187;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta266<F: Float>(t5: F, t1923: F, t2123: F, t6954: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F, t117: F, t116: F, t2126: F, t30: F, t265: F, t393: F, t2163: F, t670: F, t7193: F, t2129: F, t45: F, t606: F, t7099: F, t1209: F, t2142: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1219: F, t2134: F, t2133: F, t800: F, t1230: F, t2138: F, t1234: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7583, t7584, t7586) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1185::<F>(t5, t1923, t2123, t6954, t6960, t6963, t7566, t7576, t7579, t117, t116, t2126);
        let (t7591, t7594, t7599, t7602) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1186::<F>(t30, t265, t393, t2163, t670, t7193, t2129, t45, t606, t7099, t1209, t2142, dens_threshold, rho0, zeta_threshold);
        let (t7606, t7607) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1187::<F>(t1219, t2134, t2133, t800);
        let (t7610, t7613) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1188::<F>(t1230, t2138, t1234);
    (t7583, t7584, t7586, t7591, t7594, t7599, t7602, t7606, t7607, t7610, t7613)
}
