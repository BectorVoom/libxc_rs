//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk771;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta188<F: Float>(t1149: F, t1733: F, t3384: F, t1723: F, t3390: F, t1134: F, t3358: F, t3394: F, t5044: F, t5049: F, t5054: F, t5058: F, t1132: F, t3407: F, t1139: F, t1729: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5068, t5070, t5071, t5072, t5079) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk771::<F>(t1149, t1733, t3384, t1723, t3390, t1134, t3358, t3394, t5044, t5049, t5054, t5058);
        let (t5080, t5087, t5088, t5090, t5093) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk772::<F>(t1132, t5079, t1723, t3407, t1134, t1139, t1729, t698);
    (t5068, t5070, t5071, t5072, t5079, t5080, t5087, t5088, t5090, t5093)
}
