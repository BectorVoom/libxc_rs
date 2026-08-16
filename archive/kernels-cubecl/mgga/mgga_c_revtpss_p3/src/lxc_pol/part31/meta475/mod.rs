//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1742;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1743;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1744;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta475<F: Float>(t1927: F, t644: F, t1926: F, t531: F, t7311: F, t1962: F, t198: F, t206: F, t2411: F, t30: F, t1946: F, t2684: F, t7043: F, t820: F, t843: F, t857: F, t240: F, t7036: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25163, t25164, t25190, t25206) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1742::<F>(t1927, t644, t1926, t531, t7311, t1962, t198, t206);
        let t25207 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1743::<F>(t2411, t30);
        let (t25220, t25222) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1744::<F>(t1946, t2684, t7043, t820, t843);
        let (t25223, t25227) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1745::<F>(t25222, t857, t240, t7036);
    (t25163, t25164, t25190, t25206, t25207, t25220, t25222, t25223, t25227)
}
