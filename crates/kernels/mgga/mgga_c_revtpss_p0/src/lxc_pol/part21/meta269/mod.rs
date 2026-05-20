//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1486;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta269<F: Float>(t221: F, t3829: F, t9921: F, t3978: F, t3970: F, t3989: F, t1388: F, t3934: F, t4002: F, t5671: F, t9828: F, t9832: F, t9837: F, t9842: F, t9847: F, t9893: F, t9896: F, t9901: F, t9906: F, t9910: F, t9914: F, t9919: F, t4056: F, t550: F, t543: F, t3992: F, t2661: F, t240: F, t4000: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9923, t9924, t9926, t9928) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1486::<F>(t221, t3829, t9921, t3978, t3970, t3989, t1388, t3934, t4002, t5671, t9828, t9832, t9837, t9842, t9847, t9893, t9896, t9901, t9906, t9910, t9914, t9919);
        let (t9929, t9930, t9931, t9932, t9934) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1487::<F>(t4056, t550, t543, t3992, t2661, t240, t4000);
    (t9923, t9924, t9926, t9928, t9929, t9930, t9931, t9932, t9934)
}
