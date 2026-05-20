//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta344<F: Float>(t11239: F, t1243: F, t460: F, t3596: F, t13038: F, t1275: F, t225: F, t10270: F, t10272: F, t10279: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13126, t13127, t13141, t13142, t13147, t13148, t13180, t13181, t13182, t13261, t13262, t13263) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1356::<F>(t11239, t1243, t460, t3596, t13038, t1275, t225, t10270, t10272, t10279);
    (t13126, t13127, t13141, t13142, t13147, t13148, t13180, t13181, t13182, t13261, t13262, t13263)
}
