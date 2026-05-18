//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1027/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1027<F: Float>(t12929: F, t142: F, t12323: F, t159: F, t285: F, t532: F, t12381: F, t169: F, t301: F, t784: F, t2030: F, t3683: F) -> (F, F, F, F) {
    let t42304 = t142 * t12929;
    let t42310 = t532 * t12323 * t159 * t285;
    let t42325 = t169 * t784 * t12381 * t301;
    let t42342 = t2030 * t3683;
    (t42304, t42310, t42325, t42342)
}
