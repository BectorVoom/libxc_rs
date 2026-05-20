//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1997;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta557<F: Float>(t7059: F, t9288: F, t7064: F, t25305: F, t92868: F, t1032: F, t2760: F, t867: F, t7063: F, t7060: F, t136: F, t2457: F, t7082: F, t25299: F, t212: F, t25286: F, t689: F, t780: F, t10073: F, t1958: F, t25390: F, t886: F, t1955: F, t25308: F, t2769: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t92871, t92873, t92875, t92888, t92889, t92891, t92894) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1997::<F>(t7059, t9288, t7064, t25305, t92868, t1032, t2760, t867, t7063, t7060, t136, t2457, t7082);
        let (t92895, t92901, t92905, t92917) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1998::<F>(t25299, t92894, t212, t25286, t689, t780, t10073, t1958, t25390, t886, t1955, t25308, t2769);
    (t92871, t92873, t92875, t92888, t92889, t92891, t92894, t92895, t92901, t92905, t92917)
}
