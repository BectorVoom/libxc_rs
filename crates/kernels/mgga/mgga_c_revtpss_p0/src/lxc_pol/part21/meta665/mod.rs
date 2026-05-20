//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2462;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta665<F: Float>(t3057: F, t3316: F, t4891: F, t3298: F, t3059: F, t3154: F, t1045: F, t2853: F, t999: F, t11774: F, t127: F, t3096: F, t3128: F, t11670: F, t11772: F, t3114: F, t11773: F, t11926: F, t11858: F, t15688: F, t1020: F, t12003: F, t12077: F, t15905: F, t994: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43044, t43050, t43051, t43057, t43063) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2462::<F>(t3057, t3316, t4891, t3298, t3059, t3154, t1045, t2853, t999, t11774, t127, t3096, t3128);
        let (t43065, t43066, t43069, t43082, t43091, t43105) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2463::<F>(t11670, t11772, t3114, t11773, t11926, t11858, t15688, t1020, t12003, t12077, t15905, t994);
    (t43044, t43050, t43051, t43057, t43063, t43065, t43066, t43069, t43082, t43091, t43105)
}
