//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta300<F: Float>(t2735: F, t546: F, t1353: F, t1412: F, t808: F, t1369: F, t2699: F, t1372: F, t3943: F, t794: F, t3946: F, t159: F) -> (F, F, F, F, F, F, F, F) {
        let (t9736, t9737, t9739, t9741, t9742, t9744, t9745, t9747) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1283::<F>(t2735, t546, t1353, t1412, t808, t1369, t2699, t1372, t3943, t794, t3946, t159);
    (t9736, t9737, t9739, t9741, t9742, t9744, t9745, t9747)
}
