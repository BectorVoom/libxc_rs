//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk722;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta169<F: Float>(t1610: F, t934: F, t2874: F, t1600: F, t2880: F, t918: F, t2848: F, t2884: F, t4571: F, t4576: F, t4581: F, t4585: F, t916: F, t2897: F, t923: F, t1606: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4595, t4597, t4598, t4599, t4606) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk722::<F>(t1610, t934, t2874, t1600, t2880, t918, t2848, t2884, t4571, t4576, t4581, t4585);
        let (t4607, t4614, t4615, t4617, t4620) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk723::<F>(t4606, t916, t1600, t2897, t918, t923, t1606, t698);
    (t4595, t4597, t4598, t4599, t4606, t4607, t4614, t4615, t4617, t4620)
}
