//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1031 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1031<F: Float>(t44348: F, t52011: F, t60927: F, t44919: F, t58027: F, t3390: F, t68372: F, t141: F, t3417: F, t68290: F, t43865: F, t43888: F, t43890: F, t43892: F, t58153: F, t58158: F, t58160: F, t58162: F, t58165: F, t58186: F) -> (F, F, F, F, F, F) {
        let (t68507, t68515, t68518, t68521, t68524, t68526) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3614::<F>(t44348, t52011, t60927, t44919, t58027, t3390, t68372, t141, t3417, t68290, t43865, t43888, t43890, t43892, t58153, t58158, t58160, t58162, t58165, t58186);
    (t68507, t68515, t68518, t68521, t68524, t68526)
}
