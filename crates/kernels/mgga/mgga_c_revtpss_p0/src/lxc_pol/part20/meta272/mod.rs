//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta272<F: Float>(t140: F, t3247: F, t1011: F, t3254: F, t1015: F, t10326: F, t1012: F, t3237: F, t1014: F, t2852: F, t10356: F, t245: F, t3089: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11752, t11753, t11755, t11756, t11758, t11759, t11762, t11763, t11766, t11767, t11772) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1123::<F>(t140, t3247, t1011, t3254, t1015, t10326, t1012, t3237, t1014, t2852, t10356, t245, t3089);
    (t11752, t11753, t11755, t11756, t11758, t11759, t11762, t11763, t11766, t11767, t11772)
}
