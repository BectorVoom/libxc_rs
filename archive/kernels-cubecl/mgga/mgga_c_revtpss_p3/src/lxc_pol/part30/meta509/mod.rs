//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta509<F: Float>(t25411: F, t27186: F, t213: F, t7759: F, t25431: F, t212: F, t780: F, t689: F, t1032: F, t1568: F, t1955: F) -> (F, F, F, F, F, F, F, F) {
        let (t27187, t27189, t27192, t27194, t27195, t27196, t27198, t27199) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1890::<F>(t25411, t27186, t213, t7759, t25431, t212, t780, t689, t1032, t1568, t1955);
    (t27187, t27189, t27192, t27194, t27195, t27196, t27198, t27199)
}
