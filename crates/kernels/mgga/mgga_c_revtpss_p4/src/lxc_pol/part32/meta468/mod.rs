//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta468<F: Float>(t26292: F, t7284: F, t25878: F, t26234: F, t1445: F, t7492: F, t689: F, t1385: F, t2097: F) -> (F, F, F, F, F) {
        let (t26294, t26295, t26301, t26302, t26304) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1695::<F>(t26292, t7284, t25878, t26234, t1445, t7492, t689, t1385, t2097);
    (t26294, t26295, t26301, t26302, t26304)
}
