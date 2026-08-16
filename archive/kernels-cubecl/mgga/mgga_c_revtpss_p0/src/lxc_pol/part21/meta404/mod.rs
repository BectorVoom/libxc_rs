//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1867;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta404<F: Float>(t12256: F, t13026: F, t10356: F, t1012: F, t1204: F, t3140: F, t3599: F, t11239: F, t460: F, t1242: F, t474: F, t11243: F, t479: F, t1248: F, t3601: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13028, t13029, t13032, t13033, t13036) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1867::<F>(t12256, t13026, t10356, t1012, t1204, t3140, t3599, t11239, t460);
        let (t13037, t13038, t13039, t13040, t13041, t13042, t13043) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1868::<F>(t1242, t474, t11243, t479, t13036, t1248, t3601);
    (t13028, t13029, t13032, t13033, t13036, t13037, t13038, t13039, t13040, t13041, t13042, t13043)
}
