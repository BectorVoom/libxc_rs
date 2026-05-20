//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta673<F: Float>(t11200: F, t3286: F, t3046: F, t4995: F, t3057: F, t3143: F, t42859: F, t342: F, t16551: F, t994: F, t16558: F, t16505: F) -> (F, F, F, F, F, F, F, F) {
        let (t43446, t43453, t43456, t43471, t43472, t43520, t43524, t43528) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2409::<F>(t11200, t3286, t3046, t4995, t3057, t3143, t42859, t342, t16551, t994, t16558, t16505);
    (t43446, t43453, t43456, t43471, t43472, t43520, t43524, t43528)
}
