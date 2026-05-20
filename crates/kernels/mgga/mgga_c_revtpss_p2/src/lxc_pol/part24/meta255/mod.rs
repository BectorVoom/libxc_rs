//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta255<F: Float>(t15688: F, t3299: F, t1678: F, t3057: F, t379: F, t1078: F, t1651: F, t3286: F, t4746: F, t1647: F, t3298: F, t1086: F) -> (F, F, F, F, F, F, F) {
        let (t16226, t16284, t16312, t16313, t16502, t16509, t16543) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1023::<F>(t15688, t3299, t1678, t3057, t379, t1078, t1651, t3286, t4746, t1647, t3298, t1086);
    (t16226, t16284, t16312, t16313, t16502, t16509, t16543)
}
