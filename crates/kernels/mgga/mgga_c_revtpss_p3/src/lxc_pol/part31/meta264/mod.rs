//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1179;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1180;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1181;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1182;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1183;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta264<F: Float>(t1989: F, t3336: F, t265: F, t393: F, t207: F, t7086: F, t1940: F, t1963: F, t198: F, t2403: F, t7091: F, t775: F, t890: F, t892: F, t1100: F, t1102: F, t336: F, t5023: F, t7177: F, t30: F, t1996: F, t45: F, t606: F, t7099: F, t33: F, t1113: F, t7087: F, dens_threshold: F, rho0: F, zeta_threshold: F, t502: F, t2003: F, t57: F, rho1: F, t1936: F, t2322: F, t5523: F, t1312: F, t7002: F, t670: F, t6983: F, t6985: F, t1315: F, t196: F, t197: F, t2035: F, t2033: F, t531: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t7181 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1179::<F>(t1989, t3336);
        let (t7193, t7194) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1180::<F>(t265, t393, t207, t7086, t1940, t1963, t198, t2403, t7091, t775, t890, t892, t1100, t1102, t336, t5023, t7177, t7181);
        let (t7199, t7200, t7207, t7214) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1181::<F>(t30, t1996, t45, t606, t7099, t7194, t33, t775, t890, t1113, t1940, t1963, t2403, t7087, t7091, dens_threshold, rho0, zeta_threshold);
        let (t7215, t7221) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1182::<F>(t33, t265, t502, t7193, t2003, t57, t606, t7214, t7199, dens_threshold, rho1, zeta_threshold);
        let (t7231, t7234, t7235) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1183::<F>(t1936, t2322, t5523, t1312, t7002, t670, t6983, t6985, t1315, t196, t197);
        let (t7236, t7237) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1184::<F>(t2035, t7235, t2033, t531);
    (t7181, t7194, t7200, t7207, t7215, t7221, t7231, t7234, t7235, t7236, t7237)
}
