//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta582<F: Float>(t11735: F, t1968: F, t11772: F, t25515: F, t3114: F, t3223: F, t7131: F, t11273: F, t25504: F, t25508: F, t11263: F, t7122: F) -> (F, F, F, F, F, F, F) {
        let (t93750, t93751, t93752, t93764, t93783, t93796, t93801) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2002::<F>(t11735, t1968, t11772, t25515, t3114, t3223, t7131, t11273, t25504, t25508, t11263, t7122);
    (t93750, t93751, t93752, t93764, t93783, t93796, t93801)
}
