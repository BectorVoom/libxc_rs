//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta458<F: Float>(t4003: F, t5658: F, t1448: F, t1868: F, t2007: F, t2371: F, t197: F, t531: F, t2013: F) -> (F, F, F, F, F) {
        let (t21990, t22496, t25078, t25081, t25082) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1755::<F>(t4003, t5658, t1448, t1868, t2007, t2371, t197, t531, t2013);
    (t21990, t22496, t25078, t25081, t25082)
}
