//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1400;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta373<F: Float>(t13126: F, t460: F, t3727: F, t473: F, t11239: F, t3596: F, t13038: F, t1269: F, t3555: F, t1275: F, t225: F, t10270: F, t10272: F, t10279: F, t10281: F, t10288: F, t10290: F, t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t4171: F, t602: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13127, t13133, t13141, t13142, t13147, t13148, t13177, t13180, t13181, t13182, t13261) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1400::<F>(t13126, t460, t3727, t473, t11239, t3596, t13038, t1269, t3555, t1275, t225, t10270);
        let (t13267, t13269) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1401::<F>(t10272, t10279, t10281, t10288, t10290, t10275, t10278, t10284, t10287, t10295, t13261, t4171, t602);
    (t13127, t13133, t13141, t13142, t13147, t13148, t13177, t13180, t13181, t13182, t13267, t13269)
}
