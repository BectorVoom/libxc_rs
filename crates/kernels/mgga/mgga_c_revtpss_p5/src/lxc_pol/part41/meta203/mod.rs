//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta203<F: Float>(t1280: F, t5230: F, t1287: F, t5346: F, t1774: F, t3759: F, t5245: F, t354: F, t471: F, t1214: F, t5351: F, t3766: F, t487: F) -> (F, F, F, F, F, F, F) {
        let (t5443, t5446, t5449, t5452, t5458, t5459, t5462) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk811::<F>(t1280, t5230, t1287, t5346, t1774, t3759, t5245, t354, t471, t1214, t5351, t3766, t487);
    (t5443, t5446, t5449, t5452, t5458, t5459, t5462)
}
