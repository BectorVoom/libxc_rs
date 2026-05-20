//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta494<F: Float>(t25956: F, t26087: F, t532: F, t1450: F, t2014: F, t2042: F, t4158: F, t1459: F, t7331: F, t7334: F, t1936: F, t2327: F) -> (F, F, F, F, F, F, F, F) {
        let (t26088, t26089, t26090, t26091, t26115, t26117, t26119, t26120) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1846::<F>(t25956, t26087, t532, t1450, t2014, t2042, t4158, t1459, t7331, t7334, t1936, t2327);
    (t26088, t26089, t26090, t26091, t26115, t26117, t26119, t26120)
}
