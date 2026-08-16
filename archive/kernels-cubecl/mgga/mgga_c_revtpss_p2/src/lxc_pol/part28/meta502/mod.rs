//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1891;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta502<F: Float>(t25197: F, t26092: F, t3: F, t2042: F, t4158: F, t1459: F, t7331: F, t7334: F, t1936: F, t2327: F, t572: F, t116: F, t7002: F, param_d: F, t670: F, t2371: F, t7330: F, t117: F, t25832: F, t1461: F, t2040: F, t4162: F, t4165: F, t573: F, t7324: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26093, t26094, t26106, t26115, t26117, t26119, t26120, t26122, t26123) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1891::<F>(t25197, t26092, t3, t2042, t4158, t1459, t7331, t7334, t1936, t2327, t572, t116, t7002, param_d);
        let (t26124, t26127, t26130, t26133) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1892::<F>(t26123, t670, t572, t2371, t7330, t117, t25832, t1461, t2040, t26106, t26115, t26117, t26119, t26122, t4162, t4165, t573, t7324);
    (t26093, t26094, t26106, t26120, t26123, t26124, t26127, t26130, t26133)
}
