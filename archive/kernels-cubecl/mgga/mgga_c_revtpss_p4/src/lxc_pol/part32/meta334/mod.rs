//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1256;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta334<F: Float>(t10290: F, t4171: F, t602: F, t1466: F, t2246: F, t580: F, t9342: F, t116: F, t4245: F) -> (F, F, F, F, F, F) {
        let (t13266, t13269, t13272) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1256::<F>(t10290, t4171, t602, t1466, t2246);
        let (t13309, t13310, t13426) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1257::<F>(t580, t9342, t116, t4245);
    (t13266, t13269, t13272, t13309, t13310, t13426)
}
