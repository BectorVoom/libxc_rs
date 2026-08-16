//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk763;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta173<F: Float>(t1353: F, t30: F, t33: F, t525: F, t605: F, t2257: F, t513: F, t527: F, t1113: F, t3351: F, t516: F, t162: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
        let t3829 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk763::<F>(t1353);
        let (t3833, t3834, t3841, t3842, t3850) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk764::<F>(t30, t33, t525, t605, t2257, t513, t527, t1113, t3351, t516, t162, zeta_threshold);
    (t3829, t3833, t3834, t3841, t3842, t3850)
}
