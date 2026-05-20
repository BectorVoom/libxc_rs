//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta371<F: Float>(t1222: F, t13011: F, t140: F, t3688: F, t3700: F, t3367: F, t404: F, t1242: F, t3603: F, t471: F, t1032: F, t3552: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13012, t13014, t13015, t13017, t13018, t13026, t13038, t13045, t13068) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1398::<F>(t1222, t13011, t140, t3688, t3700, t3367, t404, t1242, t3603, t471, t1032, t3552);
    (t13012, t13014, t13015, t13017, t13018, t13026, t13038, t13045, t13068)
}
