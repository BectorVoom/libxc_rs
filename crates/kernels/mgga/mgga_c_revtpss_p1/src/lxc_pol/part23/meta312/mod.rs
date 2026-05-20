//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta312<F: Float>(t12256: F, t13026: F, t1204: F, t3140: F, t3599: F, t11239: F, t460: F, t1242: F, t474: F, t11243: F, t479: F, t3603: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13027, t13032, t13033, t13036, t13037, t13038, t13040, t13041, t13042, t13045) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1595::<F>(t12256, t13026, t1204, t3140, t3599, t11239, t460, t1242, t474, t11243, t479, t3603, t471);
    (t13027, t13032, t13033, t13036, t13037, t13038, t13040, t13041, t13042, t13045)
}
