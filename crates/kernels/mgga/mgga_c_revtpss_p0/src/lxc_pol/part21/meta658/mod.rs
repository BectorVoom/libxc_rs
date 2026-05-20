//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2449;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta658<F: Float>(t3046: F, t3298: F, t4891: F, t11263: F, t3169: F, t11977: F, t3173: F, t12009: F, t12013: F, t11916: F, t11999: F, t11874: F, t16048: F, t12046: F, t15905: F, t994: F, t3114: F, t42416: F, t11652: F, t3172: F, t4837: F, t1063: F, t11986: F, t247: F, t2862: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t42643, t42656, t42658, t42660, t42662, t42675) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2449::<F>(t3046, t3298, t4891, t11263, t3169, t11977, t3173, t12009, t12013, t11916, t11999, t11874, t16048);
        let (t42690, t42695, t42699, t42710) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2450::<F>(t12046, t15905, t994, t3114, t42416, t11652, t3172, t4837, t1063, t11986, t247, t2862);
    (t42643, t42656, t42658, t42660, t42662, t42675, t42690, t42695, t42699, t42710)
}
