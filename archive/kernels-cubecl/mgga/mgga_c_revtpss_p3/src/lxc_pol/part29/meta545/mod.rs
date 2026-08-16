//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1881;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta545<F: Float>(t26072: F, t26292: F, t7493: F, t9292: F, t136: F, t137: F, t2097: F, t94386: F, t94391: F, t1358: F, t212: F, t26333: F, t689: F, t9646: F, t9648: F, t7515: F, t94894: F, t25899: F, t96192: F, t25875: F, t96186: F, t94398: F, t3916: F, t96191: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t96211, t96218, t96220, t96221, t96222, t96226) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1881::<F>(t26072, t26292, t7493, t9292, t136, t137, t2097, t94386, t94391, t1358, t212, t26333, t689);
        let (t96230, t96232, t96234, t96236, t96237, t96239) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1882::<F>(t2097, t9646, t9648, t7515, t94894, t25899, t96192, t25875, t96186, t94398, t3916, t96191);
    (t96211, t96218, t96220, t96221, t96222, t96226, t96230, t96232, t96234, t96236, t96237, t96239)
}
