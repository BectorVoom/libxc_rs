//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta443<F: Float>(t11262: F, t1796: F, t1247: F, t1264: F, t16746: F, t247: F, t12915: F, t5230: F, t5384: F, t1770: F, t3140: F, t3609: F) -> (F, F, F, F, F, F, F) {
        let (t17361, t17362, t17369, t17373, t17375, t17376, t17377) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1696::<F>(t11262, t1796, t1247, t1264, t16746, t247, t12915, t5230, t5384, t1770, t3140, t3609);
    (t17361, t17362, t17369, t17373, t17375, t17376, t17377)
}
