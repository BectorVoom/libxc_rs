//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2136;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2137;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta648(t29598: f64, t775: f64, t25207: f64, t1940: f64, t2255: f64, t7783: f64, t77425: f64, t106498: f64, t106502: f64, t106510: f64, t106516: f64, t106520: f64, t106528: f64, t1468: f64, t2403: f64, t25206: f64, t27158: f64, t27166: f64, t27173: f64, t27364: f64, t27368: f64, t27391: f64, t29705: f64, t605: f64, t7091: f64, t7092: f64, t7787: f64, t98637: f64, t99555: f64, t4433: f64, t892: f64, t1583: f64, t4537: f64, t27383: f64, t6079: f64, t890: f64, t98785: f64, t77408: f64, t18498: f64, t27159: f64, t25440: f64, t27382: f64, t27395: f64, t27402: f64, t29592: f64, t29606: f64, t29713: f64, t29719: f64, t50080: f64, t7087: f64, t7749: f64, t93404: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t106533, t106539, t106543) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2136(t29598, t775, t25207, t1940, t2255, t7783, t77425, t106498, t106502, t106510, t106516, t106520, t106528, t1468, t2403, t25206, t27158, t27166, t27173, t27364, t27368, t27391, t29705, t605, t7091, t7092, t7787, t98637, t99555);
        let (t106546, t106554, t106555, t106561, t106562, t106565, t106566, t106569) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2137(t1468, t4433, t892, t1583, t4537, t27383, t6079, t775, t890, t98785, t25207, t77408);
        let t106588 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2138(t18498, t27159, t1468, t4537, t106546, t106555, t106562, t106566, t106569, t1940, t2403, t25206, t25440, t27158, t27364, t27368, t27382, t27395, t27402, t29592, t29606, t29713, t29719, t50080, t7087, t7091, t7749, t7783, t93404);
    (t106533, t106539, t106543, t106554, t106561, t106565, t106588)
}
