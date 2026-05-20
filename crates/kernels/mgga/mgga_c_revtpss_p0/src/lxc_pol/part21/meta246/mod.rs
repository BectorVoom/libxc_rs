//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1426;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta246<F: Float>(t1340: F, t9425: F, t1330: F, t2608: F, t512: F, t169: F, t2552: F, t164: F, t2538: F, t729: F, t2556: F, t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9427, t9428, t9429, t9430, t9432, t9433, t9434, t9435, t9446) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1426::<F>(t1340, t9425, t1330, t2608, t512, t169, t2552, t164, t2538, t729, t2556, t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303);
    (t9427, t9428, t9429, t9430, t9432, t9433, t9434, t9435, t9446)
}
