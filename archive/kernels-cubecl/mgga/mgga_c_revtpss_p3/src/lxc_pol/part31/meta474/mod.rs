//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1740;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta474<F: Float>(t4003: F, t6843: F, t2723: F, t6016: F, t197: F, t531: F, t2013: F, t10301: F, t6957: F, t38: F, t6972: F, t2247: F, t48: F, t613: F, t2275: F, t43: F, t239: F, t10309: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23037, t23160, t25081, t25082) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1740::<F>(t4003, t6843, t2723, t6016, t197, t531, t2013);
        let (t25099, t25105, t25106, t25129, t25132, t25137, t25157, t25162) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1741::<F>(t10301, t6957, t38, t6972, t2247, t48, t613, t2275, t43, t239, t10309, t607);
    (t23037, t23160, t25081, t25082, t25099, t25105, t25106, t25129, t25132, t25137, t25157, t25162)
}
