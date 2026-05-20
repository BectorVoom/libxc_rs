//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta765<F: Float>(t342: F, t43471: F, t3043: F, t3298: F, t16551: F, t994: F, t16558: F, t16505: F, t11627: F, t42859: F, t16553: F, t3133: F) -> (F, F, F, F, F, F, F, F) {
        let (t43472, t43512, t43520, t43524, t43528, t43536, t43537, t43568) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2847::<F>(t342, t43471, t3043, t3298, t16551, t994, t16558, t16505, t11627, t42859, t16553, t3133);
    (t43472, t43512, t43520, t43524, t43528, t43536, t43537, t43568)
}
