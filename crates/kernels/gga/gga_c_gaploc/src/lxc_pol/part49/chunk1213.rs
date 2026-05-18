//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1213/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1213<F: Float>(t12277: F, t2728: F, t44232: F, t44234: F, t44236: F, t44238: F, t44239: F, t44242: F, t44243: F, t44245: F, t47790: F, t47791: F, t48241: F) -> F {
    let t48242 = t12277 * t2728;
    let t48243 = -t44232 - t44234 + t47790 + t44236 + t44238 - t44239 + t47791 + t48241 + t44242 - t48242 + t44243 + t44245;
    t48243
}
