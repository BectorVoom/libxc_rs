//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1021;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta254<F: Float>(t140: F, t3252: F, t1012: F, t11821: F, t12047: F, t15905: F, t12167: F, t3057: F, t380: F, t3088: F, t370: F, t994: F, t1651: F, t3181: F, t11150: F, t11144: F, t11852: F, t1655: F, t697: F, t1011: F, t372: F, t4806: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15993, t16012, t16067, t16081, t16088, t16089, t16094) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1021::<F>(t140, t3252, t1012, t11821, t12047, t15905, t12167, t3057, t380, t3088, t370, t994);
        let (t16095, t16170, t16199, t16208, t16220, t16222) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1022::<F>(t16088, t16094, t1651, t3181, t11150, t11144, t11852, t1655, t697, t1011, t372, t4806);
    (t15993, t16012, t16067, t16081, t16089, t16095, t16170, t16199, t16208, t16220, t16222)
}
