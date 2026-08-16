//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1899;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta512<F: Float>(t1955: F, t4469: F, t72: F, t7778: F, t686: F, t7064: F, t1558: F, t231: F, t7048: F, t7076: F, t1949: F, t4423: F, t1959: F, t25297: F, t25303: F, t25307: F, t25311: F, t25333: F, t25337: F, t25340: F, t25353: F, t25356: F, t25383: F, t7070: F, t7775: F) -> (F, F, F, F, F, F, F, F) {
        let (t27275, t27278, t27279, t27280, t27286, t27287, t27291) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1899::<F>(t1955, t4469, t72, t7778, t686, t7064, t1558, t231, t7048, t7076, t1949, t4423);
        let (t27292, t27297) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1900::<F>(t27291, t7076, t1959, t25297, t25303, t25307, t25311, t25333, t25337, t25340, t25353, t25356, t25383, t27275, t27280, t27287, t7070, t7775);
    (t27275, t27278, t27279, t27286, t27287, t27291, t27292, t27297)
}
