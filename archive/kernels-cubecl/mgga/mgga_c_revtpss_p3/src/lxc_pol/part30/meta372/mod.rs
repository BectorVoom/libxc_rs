//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta372<F: Float>(t1246: F, t13068: F, t247: F, t3372: F, t3634: F, t1261: F, t3368: F, t3636: F, t3647: F, t3367: F, t414: F, t11239: F, t1243: F) -> (F, F, F, F, F, F, F, F) {
        let (t13069, t13085, t13086, t13089, t13090, t13092, t13099, t13126) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1399::<F>(t1246, t13068, t247, t3372, t3634, t1261, t3368, t3636, t3647, t3367, t414, t11239, t1243);
    (t13069, t13085, t13086, t13089, t13090, t13092, t13099, t13126)
}
