//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1016;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1017;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta277<F: Float>(t1386: F, t2681: F, t820: F, t1401: F, t4000: F, t843: F, t4006: F, t136: F, t4011: F, t221: F, t3829: F, t3978: F, t3970: F, t3989: F, t4056: F, t550: F, t543: F, t3992: F, t2661: F, t240: F, t4003: F, t9768: F, t532: F, t549: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9909, t9910, t9919, t9921, t9924) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1016::<F>(t1386, t2681, t820, t1401, t4000, t843, t4006, t136, t4011, t221, t3829, t3978);
        let (t9926, t9932, t9934, t9937, t9940) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1017::<F>(t3970, t3989, t4056, t550, t543, t3992, t2661, t240, t4000, t4003, t9768, t532, t549);
    (t9909, t9910, t9919, t9921, t9924, t9926, t9932, t9934, t9937, t9940)
}
