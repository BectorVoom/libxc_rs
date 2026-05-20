//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1282;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta352<F: Float>(t10578: F, t9575: F, t9572: F, t2434: F, t2496: F, t2629: F, t676: F, t9419: F, t9866: F, t123: F, t2390: F, t2630: F, t9863: F, t762: F, t9291: F, t2251: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39424, t39426, t39427, t39429, t39430, t39432, t39434, t39436) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1282::<F>(t10578, t9575, t9572, t2434, t2496, t2629, t676, t9419, t9866, t123, t2390, t2630);
        let (t39437, t39439, t39440, t39442, t39443) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1283::<F>(t39436, t10578, t9863, t762, t9291, t2629, t2251);
    (t39424, t39426, t39427, t39429, t39430, t39432, t39434, t39437, t39439, t39440, t39442, t39443)
}
