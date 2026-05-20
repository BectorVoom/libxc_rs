//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1796;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1797;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta474<F: Float>(t2467: F, t25399: F, t233: F, t867: F, t1949: F, t7056: F, t10073: F, t1955: F, t2760: F, t1957: F, t822: F, t25386: F, t676: F, t837: F, t25377: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25400, t25402, t25403, t25404, t25406, t25407, t25410) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1796::<F>(t2467, t25399, t233, t867, t1949, t7056, t10073, t1955, t2760, t1957, t822);
        let t25411 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1797::<F>(t25386, t25410);
        let (t25412, t25413) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1798::<F>(t676, t837, t25377);
    (t25400, t25402, t25403, t25404, t25406, t25407, t25410, t25411, t25412, t25413)
}
