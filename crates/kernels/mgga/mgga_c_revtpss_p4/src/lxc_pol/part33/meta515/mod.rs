//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1850;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1851;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta515<F: Float>(t27383: F, t27384: F, t1583: F, t605: F, t30: F, t4537: F, t1468: F, t775: F, t890: F, t1940: F, t1963: F, t2255: F, t2403: F, t25206: F, t25440: F, t27158: F, t27160: F, t27166: F, t27169: F, t27173: F, t27364: F, t27368: F, t27376: F, t27382: F, t7010: F, t7087: F, t7091: F, t7092: F, t7749: F, t7783: F, t7787: F, t1544: F, t18875: F, t198: F, t207: F, t25445: F, t27363: F, t27375: F, t4343: F, t4433: F, t4541: F, t892: F) -> (F, F, F, F, F, F, F, F) {
        let (t27385, t27387, t27391, t27395, t27402, t27407) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1850::<F>(t27383, t27384, t1583, t605, t30, t4537, t1468, t775, t890, t1940, t1963, t2255);
        let t27408 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1851::<F>(t1468, t1940, t1963, t2403, t25206, t25440, t27158, t27160, t27166, t27169, t27173, t27364, t27368, t27376, t27382, t27385, t27387, t27391, t27395, t27402, t27407, t30, t605, t7010, t7087, t7091, t7092, t7749, t7783, t7787);
        let t27754 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1852::<F>(t1544, t1583, t18875, t1940, t1963, t198, t207, t2403, t25440, t25445, t27363, t27368, t27375, t27384, t4343, t4433, t4537, t4541, t7087, t7091, t775, t7783, t890, t892);
    (t27385, t27387, t27391, t27395, t27402, t27407, t27408, t27754)
}
