//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk929;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta246<F: Float>(t1868: F, t566: F, t198: F, t532: F, t1907: F, t4147: F, t1317: F, t1857: F, t1320: F, t1468: F, t3833: F, t2: F, t513: F, t30: F, t33: F, t580: F, t605: F, t1711: F, t3841: F, t516: F, t1113: F, t162: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5537, t5541, t5542, t5546, t5548, t5549, t5552) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk929::<F>(t1868, t566, t198, t532, t1907, t4147, t1317, t1857, t1320, t1468, t3833, t2, t513);
        let (t5557, t5560, t5566) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk930::<F>(t30, t33, t5549, t5552, t580, t605, t1711, t3841, t2, t516, t1113, t162, zeta_threshold);
    (t5537, t5541, t5542, t5546, t5548, t5549, t5552, t5557, t5560, t5566)
}
