//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1770;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1771;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1772;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta484<F: Float>(t676: F, t837: F, t25377: F, t25411: F, t2718: F, t867: F, t1950: F, t2453: F, t2458: F, t25372: F, t25410: F, t2411: F, t7086: F, t11064: F, t1962: F, t1032: F, t1071: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25412, t25413, t25414, t25416, t25422, t25424, t25431) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1770::<F>(t676, t837, t25377, t25411, t2718, t867, t1950, t2453, t2458, t25372, t25410);
        let (t25432, t25440) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1771::<F>(t25413, t25431, t2411, t7086);
        let t25445 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1772::<F>(t11064, t1962);
        let t25460 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1773::<F>(t1032, t1071);
    (t25412, t25413, t25414, t25416, t25422, t25424, t25431, t25432, t25440, t25445, t25460)
}
