//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1236;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1237;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1238;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta276(t265: f64, t393: f64, t1544: f64, t1963: f64, t207: f64, t7782: f64, t1583: f64, t1940: f64, t198: f64, t2403: f64, t7091: f64, t892: f64, t1102: f64, t1699: f64, t336: f64, t5023: f64, t7181: f64, t7840: f64, t30: f64, t1469: f64, t1996: f64, t45: f64, t7794: f64, t33: f64, t1711: f64, t7783: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t502: f64, t2003: f64, t57: f64, rho1: f64, t1936: f64, t4248: f64, t1518: f64, t93: f64, t1312: f64, t7741: f64, t6985: f64, t7725: f64, t1847: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7847, t7855, t7856) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1236(t265, t393, t1544, t1963, t207, t7782, t1583, t1940, t198, t2403, t7091, t892, t1102, t1699, t336, t5023, t7181, t7840);
        let (t7861, t7862, t7863, t7869, t7876) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1237(t30, t1469, t1996, t45, t7794, t7856, t1544, t33, t1963, t1583, t1711, t1940, t2403, t7091, t7783, dens_threshold, rho0, zeta_threshold);
        let (t7877, t7883) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1238(t33, t265, t502, t7855, t1469, t2003, t57, t7876, t7861, dens_threshold, rho1, zeta_threshold);
        let (t7889, t7894, t7897, t7898) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1239(t1936, t4248, t1518, t93, t1312, t7741, t6985, t7725, t1847, t196, t197);
    (t7847, t7856, t7862, t7863, t7869, t7877, t7883, t7889, t7894, t7897, t7898)
}
