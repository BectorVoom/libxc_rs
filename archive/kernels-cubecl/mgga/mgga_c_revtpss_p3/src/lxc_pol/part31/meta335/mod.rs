//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta335<F: Float>(t12485: F, t439: F, t1175: F, t3495: F, t1156: F, t3451: F, t12295: F, t12351: F, t1178: F, t3519: F, t3522: F, t447: F) -> (F, F, F, F, F, F, F, F) {
        let (t12486, t12491, t12511, t12542, t12543, t12552, t12553, t12555) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1340::<F>(t12485, t439, t1175, t3495, t1156, t3451, t12295, t12351, t1178, t3519, t3522, t447);
    (t12486, t12491, t12511, t12542, t12543, t12552, t12553, t12555)
}
