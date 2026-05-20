//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1980;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta568<F: Float>(t7059: F, t9288: F, t7064: F, t25305: F, t92868: F, t136: F, t2457: F, t7082: F, t25299: F, t10073: F, t1958: F, t25390: F, t886: F, t1955: F, t25308: F, t2769: F, t7049: F, t786: F, t867: F, t2439: F, t25334: F, t887: F, t7036: F, t820: F, t844: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t92871, t92873, t92875, t92894, t92895, t92905) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1980::<F>(t7059, t9288, t7064, t25305, t92868, t136, t2457, t7082, t25299, t10073, t1958, t25390, t886);
        let (t92917, t92921, t92935, t92951) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1981::<F>(t1955, t25308, t2769, t7049, t786, t867, t2439, t25334, t887, t7036, t820, t844);
    (t92871, t92873, t92875, t92894, t92895, t92905, t92917, t92921, t92935, t92951)
}
