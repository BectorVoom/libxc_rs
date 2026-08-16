//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2735;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta710<F: Float>(t10578: F, t9575: F, t9572: F, t2434: F, t2496: F, t2629: F, t676: F, t9419: F, t9866: F, t123: F, t2390: F, t2630: F) -> (F, F, F, F, F, F, F, F) {
        let (t39423, t39425, t39427, t39429, t39430, t39432, t39433, t39436) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2735::<F>(t10578, t9575, t9572, t2434, t2496, t2629, t676, t9419, t9866, t123, t2390, t2630);
    (t39423, t39425, t39427, t39429, t39430, t39432, t39433, t39436)
}
