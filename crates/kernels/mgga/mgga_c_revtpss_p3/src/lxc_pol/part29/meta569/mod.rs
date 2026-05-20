//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta569<F: Float>(t1468: F, t2832: F, t2408: F, t25207: F, t61182: F, t2430: F, t1583: F, t2257: F, t2394: F, t11064: F, t605: F, t27384: F) -> (F, F, F, F, F, F, F, F) {
        let (t98736, t98740, t98743, t98751, t98755, t98759, t98760, t98764) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1916::<F>(t1468, t2832, t2408, t25207, t61182, t2430, t1583, t2257, t2394, t11064, t605, t27384);
    (t98736, t98740, t98743, t98751, t98755, t98759, t98760, t98764)
}
