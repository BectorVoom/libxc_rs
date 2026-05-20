//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta565<F: Float>(t10997: F, t93261: F, t25404: F, t40270: F, t10510: F, t25399: F, t10115: F, t1951: F, t7058: F, t92871: F, t1032: F, t11007: F) -> (F, F, F, F, F, F) {
        let (t93262, t93272, t93273, t93276, t93278, t93279) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2024::<F>(t10997, t93261, t25404, t40270, t10510, t25399, t10115, t1951, t7058, t92871, t1032, t11007);
    (t93262, t93272, t93273, t93276, t93278, t93279)
}
