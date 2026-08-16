//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3045/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3045<F: Float>(t1298: F, t5023: F, t81128: F, t81130: F, t81132: F, t81134: F, t81136: F, t81138: F, t81139: F, t81145: F, t81148: F, t81150: F, t81152: F) -> F {
    let t81153 = -t1298 * t5023 * t81139 + t81128 + t81130 + t81132 + t81134 + t81136 - t81138 - t81145 + t81148 - t81150 + t81152;
    t81153
}
