//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta192<F: Float>(t4056: F, t550: F, t543: F, t3992: F, t2661: F, t240: F, t4000: F, t4003: F, t9768: F, t532: F, t549: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9929, t9930, t9931, t9932, t9934, t9935, t9936, t9937, t9940, t9941, t9942) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk952::<F>(t4056, t550, t543, t3992, t2661, t240, t4000, t4003, t9768, t532, t549, t72);
    (t9929, t9930, t9931, t9932, t9934, t9935, t9936, t9937, t9940, t9941, t9942)
}
