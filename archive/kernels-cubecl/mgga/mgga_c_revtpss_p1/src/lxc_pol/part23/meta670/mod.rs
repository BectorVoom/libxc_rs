//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2404;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta670<F: Float>(t1041: F, t1046: F, t42994: F, t3057: F, t3316: F, t4891: F, t3298: F, t11670: F, t11772: F, t3114: F, t11773: F, t11926: F, t11858: F, t15688: F, t12077: F, t15905: F, t994: F, t11725: F, t828: F, t225: F, t42059: F, t1053: F, t11940: F, t11240: F, t11628: F, t42646: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42996, t43044, t43050, t43066, t43069) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2404::<F>(t1041, t1046, t42994, t3057, t3316, t4891, t3298, t11670, t11772, t3114, t11773, t11926);
        let (t43082, t43105, t43131, t43154, t43161, t43207) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2405::<F>(t11858, t15688, t12077, t15905, t994, t11725, t828, t225, t42059, t1053, t11940, t11240, t11628, t42646);
    (t42996, t43044, t43050, t43066, t43069, t43082, t43105, t43131, t43154, t43161, t43207)
}
