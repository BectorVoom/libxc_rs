//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1218;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta315<F: Float>(t2496: F, t2523: F, t760: F, t9372: F, t37: F, t716: F, t2626: F, t9425: F, t2609: F, t606: F, t706: F, t775: F, t853: F, t2710: F, t2793: F, t9285: F, t2470: F, t2804: F, t874: F, t875: F, t9288: F, t251: F, t2722: F, t2723: F, t4503: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10597, t10604, t10605, t10608, t10611, t10613, t10631) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1218::<F>(t2496, t2523, t760, t9372, t37, t716, t2626, t9425, t2609, t606, t706, t775, t853);
        let (t10645, t10647, t10651, t10652, t10654) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1219::<F>(t2710, t2793, t9285, t2470, t2804, t874, t875, t9288, t251, t2722, t2723, t4503);
    (t10597, t10604, t10605, t10608, t10611, t10613, t10631, t10645, t10647, t10651, t10652, t10654)
}
