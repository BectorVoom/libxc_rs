//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2136;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2137;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta648<F: Float>(t29598: F, t775: F, t25207: F, t1940: F, t2255: F, t7783: F, t77425: F, t106498: F, t106502: F, t106510: F, t106516: F, t106520: F, t106528: F, t1468: F, t2403: F, t25206: F, t27158: F, t27166: F, t27173: F, t27364: F, t27368: F, t27391: F, t29705: F, t605: F, t7091: F, t7092: F, t7787: F, t98637: F, t99555: F, t4433: F, t892: F, t1583: F, t4537: F, t27383: F, t6079: F, t890: F, t98785: F, t77408: F, t18498: F, t27159: F, t25440: F, t27382: F, t27395: F, t27402: F, t29592: F, t29606: F, t29713: F, t29719: F, t50080: F, t7087: F, t7749: F, t93404: F) -> (F, F, F, F, F, F, F) {
        let (t106533, t106539, t106543) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2136::<F>(t29598, t775, t25207, t1940, t2255, t7783, t77425, t106498, t106502, t106510, t106516, t106520, t106528, t1468, t2403, t25206, t27158, t27166, t27173, t27364, t27368, t27391, t29705, t605, t7091, t7092, t7787, t98637, t99555);
        let (t106546, t106554, t106555, t106561, t106562, t106565, t106566, t106569) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2137::<F>(t1468, t4433, t892, t1583, t4537, t27383, t6079, t775, t890, t98785, t25207, t77408);
        let t106588 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2138::<F>(t18498, t27159, t1468, t4537, t106546, t106555, t106562, t106566, t106569, t1940, t2403, t25206, t25440, t27158, t27364, t27368, t27382, t27395, t27402, t29592, t29606, t29713, t29719, t50080, t7087, t7091, t7749, t7783, t93404);
    (t106533, t106539, t106543, t106554, t106561, t106565, t106588)
}
