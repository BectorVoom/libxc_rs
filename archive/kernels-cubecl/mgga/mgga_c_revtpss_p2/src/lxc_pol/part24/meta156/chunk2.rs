//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 788/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk788<F: Float>(t6106: F, t6108: F, t6112: F, t6144: F, t6147: F, t6213: F, t6215: F, t6217: F, t6221: F, t6225: F, t6229: F) -> F {
    let t6299 = -t6106 + t6108 - t6112 + t6144 + t6147 + t6213 + t6215 - t6217 + t6221 - t6225 - t6229;
    t6299
}
