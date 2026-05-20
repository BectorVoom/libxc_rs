//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1193;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1194;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1195;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta266<F: Float>(t30: F, t1996: F, t45: F, t606: F, t7099: F, t7194: F, t33: F, t775: F, t890: F, t1113: F, t1940: F, t1963: F, t2403: F, t7087: F, t7091: F, dens_threshold: F, rho0: F, zeta_threshold: F, t265: F, t502: F, t7193: F, t2003: F, t57: F, rho1: F, t1936: F, t2322: F, t5523: F, t1312: F, t7002: F, t670: F, t6983: F, t6985: F, t1315: F, t196: F, t197: F, t2035: F, t2033: F, t531: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7199, t7200, t7207, t7214) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1193::<F>(t30, t1996, t45, t606, t7099, t7194, t33, t775, t890, t1113, t1940, t1963, t2403, t7087, t7091, dens_threshold, rho0, zeta_threshold);
        let (t7215, t7221) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1194::<F>(t33, t265, t502, t7193, t2003, t57, t606, t7214, t7199, dens_threshold, rho1, zeta_threshold);
        let (t7231, t7234, t7235) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1195::<F>(t1936, t2322, t5523, t1312, t7002, t670, t6983, t6985, t1315, t196, t197);
        let (t7236, t7237) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1196::<F>(t2035, t7235, t2033, t531);
    (t7200, t7207, t7215, t7221, t7231, t7234, t7235, t7236, t7237)
}
