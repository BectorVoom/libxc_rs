//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta714 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta714<F: Float>(t3863: F, t5567: F, t3857: F, t2608: F, t512: F, t5566: F, t1856: F, t9544: F, t46975: F, t46979: F, t13597: F, t2516: F) -> (F, F, F, F, F, F, F) {
        let (t48234, t48236, t48241, t48243, t48244, t48248, t48255) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2473::<F>(t3863, t5567, t3857, t2608, t512, t5566, t1856, t9544, t46975, t46979, t13597, t2516);
    (t48234, t48236, t48241, t48243, t48244, t48248, t48255)
}
