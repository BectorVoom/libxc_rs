//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta326<F: Float>(t10356: F, t13020: F, t1012: F, t3367: F, t404: F, t12256: F, t1204: F, t3140: F, t3599: F, t11239: F, t460: F, t1242: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13021, t13022, t13026, t13028, t13029, t13032, t13033, t13036, t13037) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1241::<F>(t10356, t13020, t1012, t3367, t404, t12256, t1204, t3140, t3599, t11239, t460, t1242);
    (t13021, t13022, t13026, t13028, t13029, t13032, t13033, t13036, t13037)
}
