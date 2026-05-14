//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1358/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1358<F: Float>(t105964: F, t105977: F, t105991: F, t106004: F, t106018: F, t106031: F, t106045: F, t106058: F, t106072: F, t106085: F, t106099: F, t106111: F, t106125: F, t106138: F, t106152: F, t106163: F) -> (F,) {
    let t106167 = t105964 + t105977 + t105991 + t106004 + t106018 + t106031 + t106045 + t106058 + t106072 + t106085 + t106099 + t106111 + t106125 + t106138 + t106152 + t106163;
    (t106167,)
}
