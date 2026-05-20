//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta755 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2831;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta755<F: Float>(t3105: F, t3223: F, t1041: F, t11262: F, t3135: F, t12166: F, t15905: F, t994: F, t11631: F, t999: F, t3046: F, t3298: F, t4891: F, t11263: F, t3169: F, t3043: F, t3140: F, t3149: F, t3160: F, t11874: F, t16048: F, t12046: F, t3114: F, t42416: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42571, t42580, t42621, t42622, t42643) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2831::<F>(t3105, t3223, t1041, t11262, t3135, t12166, t15905, t994, t11631, t999, t3046, t3298, t4891);
        let (t42656, t42665, t42672, t42675, t42690, t42695) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2832::<F>(t11263, t3169, t3043, t3140, t3149, t3160, t11874, t16048, t12046, t15905, t994, t3114, t42416);
    (t42571, t42580, t42621, t42622, t42643, t42656, t42665, t42672, t42675, t42690, t42695)
}
