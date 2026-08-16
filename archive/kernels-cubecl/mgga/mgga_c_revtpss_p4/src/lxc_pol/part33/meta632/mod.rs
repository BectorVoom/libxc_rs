//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta632<F: Float>(t25387: F, t99349: F, t2470: F, t27340: F, t7063: F, t99271: F, t7060: F, t136: F, t2457: F, t7778: F, t25299: F, t25412: F, t99348: F) -> (F, F, F, F, F, F, F) {
        let (t99351, t99365, t99366, t99375, t99380, t99381, t99389) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2079::<F>(t25387, t99349, t2470, t27340, t7063, t99271, t7060, t136, t2457, t7778, t25299, t25412, t99348);
    (t99351, t99365, t99366, t99375, t99380, t99381, t99389)
}
