//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta587<F: Float>(t3302: F, t357: F, t4982: F, t999: F, t1647: F, t4980: F, t4995: F, t1678: F, t3298: F, t342: F, t3316: F, t1045: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19502, t19526, t19569, t19579, t19602, t19603, t19607, t19608, t19620) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2304::<F>(t3302, t357, t4982, t999, t1647, t4980, t4995, t1678, t3298, t342, t3316, t1045);
    (t19502, t19526, t19569, t19579, t19602, t19603, t19607, t19608, t19620)
}
