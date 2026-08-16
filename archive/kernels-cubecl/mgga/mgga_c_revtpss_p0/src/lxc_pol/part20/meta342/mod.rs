//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1268;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta342<F: Float>(t12167: F, t15905: F, t3057: F, t380: F, t3088: F, t370: F, t994: F, t905: F, t999: F, t606: F, t1045: F, t11150: F, t3181: F, t11144: F, t11852: F, t15688: F, t3299: F, t1043: F, t3155: F, t379: F, t1071: F, t3298: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16081, t16089, t16095, t16101, t16102, t16103, t16199) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1268::<F>(t12167, t15905, t3057, t380, t3088, t370, t994, t905, t999, t606, t1045, t11150, t3181);
        let (t16208, t16226, t16229, t16312, t16409) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1269::<F>(t11144, t11852, t15688, t3299, t1043, t905, t606, t3155, t3057, t379, t1071, t3298);
    (t16081, t16089, t16095, t16101, t16102, t16103, t16199, t16208, t16226, t16229, t16312, t16409)
}
