//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta274<F: Float>(t10578: F, t2630: F, t2629: F, t9866: F, t9575: F, t9572: F, t760: F, t9419: F, t2516: F, t2523: F, t9387: F, t2496: F) -> (F, F, F, F, F, F, F, F) {
        let (t10579, t10582, t10584, t10586, t10592, t10593, t10596, t10597) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1488::<F>(t10578, t2630, t2629, t9866, t9575, t9572, t760, t9419, t2516, t2523, t9387, t2496);
    (t10579, t10582, t10584, t10586, t10592, t10593, t10596, t10597)
}
