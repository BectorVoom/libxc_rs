//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1850;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1851;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta515(t27383: f64, t27384: f64, t1583: f64, t605: f64, t30: f64, t4537: f64, t1468: f64, t775: f64, t890: f64, t1940: f64, t1963: f64, t2255: f64, t2403: f64, t25206: f64, t25440: f64, t27158: f64, t27160: f64, t27166: f64, t27169: f64, t27173: f64, t27364: f64, t27368: f64, t27376: f64, t27382: f64, t7010: f64, t7087: f64, t7091: f64, t7092: f64, t7749: f64, t7783: f64, t7787: f64, t1544: f64, t18875: f64, t198: f64, t207: f64, t25445: f64, t27363: f64, t27375: f64, t4343: f64, t4433: f64, t4541: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27385, t27387, t27391, t27395, t27402, t27407) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1850(t27383, t27384, t1583, t605, t30, t4537, t1468, t775, t890, t1940, t1963, t2255);
        let t27408 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1851(t1468, t1940, t1963, t2403, t25206, t25440, t27158, t27160, t27166, t27169, t27173, t27364, t27368, t27376, t27382, t27385, t27387, t27391, t27395, t27402, t27407, t30, t605, t7010, t7087, t7091, t7092, t7749, t7783, t7787);
        let t27754 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1852(t1544, t1583, t18875, t1940, t1963, t198, t207, t2403, t25440, t25445, t27363, t27368, t27375, t27384, t4343, t4433, t4537, t4541, t7087, t7091, t775, t7783, t890, t892);
    (t27385, t27387, t27391, t27395, t27402, t27407, t27408, t27754)
}
