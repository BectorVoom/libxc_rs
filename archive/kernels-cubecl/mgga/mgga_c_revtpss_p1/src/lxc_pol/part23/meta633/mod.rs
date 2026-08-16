//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta633<F: Float>(t10578: F, t9575: F, t9572: F, t2434: F, t2496: F, t2629: F, t676: F, t9419: F, t9866: F, t9863: F, t762: F, t9291: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39423, t39425, t39427, t39429, t39430, t39432, t39433, t39438, t39440) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2329::<F>(t10578, t9575, t9572, t2434, t2496, t2629, t676, t9419, t9866, t9863, t762, t9291);
    (t39423, t39425, t39427, t39429, t39430, t39432, t39433, t39438, t39440)
}
