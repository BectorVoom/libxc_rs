//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1225;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta277(t1583: f64, t1940: f64, t198: f64, t2403: f64, t7091: f64, t7847: f64, t7850: f64, t892: f64, t1544: f64, t33: f64, t1963: f64, t1711: f64, t7783: f64, t1936: f64, t4248: f64, t1518: f64, t93: f64, t1312: f64, t7741: f64, t1847: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7855, t7862, t7863, t7869, t7876) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1225(t1583, t1940, t198, t2403, t7091, t7847, t7850, t892, t1544, t33, t1963, t1711, t7783);
        let (t7888, t7889, t7891, t7893, t7897, t7898) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1226(t1936, t4248, t1518, t93, t1312, t7741, t1847, t196, t197);
    (t7855, t7862, t7863, t7869, t7876, t7888, t7889, t7891, t7893, t7897, t7898)
}
