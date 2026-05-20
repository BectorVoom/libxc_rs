//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2458;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta663<F: Float>(t11875: F, t11876: F, t11922: F, t11991: F, t3111: F, t1062: F, t11903: F, t11988: F, t3188: F, t11263: F, t3124: F, t11262: F, t3150: F, t3156: F, t3161: F, t3163: F, t11267: F, t3123: F, t12016: F, t3115: F, t11638: F, t3127: F, t3172: F, t11683: F, t11710: F, t3091: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42900, t42902, t42904, t42907, t42926, t42929) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2458::<F>(t11875, t11876, t11922, t11991, t3111, t1062, t11903, t11988, t3188, t11263, t3124, t11262, t3150, t3156);
        let (t42932, t42934, t42947, t42962, t42965) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2459::<F>(t11262, t3161, t3163, t11267, t3123, t11922, t12016, t3115, t11638, t3127, t3172, t11683, t11710, t3091);
    (t42900, t42902, t42904, t42907, t42926, t42929, t42932, t42934, t42947, t42962, t42965)
}
