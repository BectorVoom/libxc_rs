//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1026/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1026<F: Float>(t12277: F, t2728: F, t44232: F, t44234: F, t44236: F, t44238: F, t44239: F, t44242: F, t44243: F, t44245: F, t47790: F, t47791: F, t48241: F, t224: F, t42496: F, t42501: F, t42503: F, t42506: F, t42509: F, t42513: F, t47074: F, t47075: F, t47078: F, t47089: F, t47092: F, t47095: F, t47098: F, t47108: F, t47109: F, t47110: F, t47124: F, t47789: F) -> (F,) {
    let t48242 = t12277 * t2728;
    let t48243 = -t44232 - t44234 + t47790 + t44236 + t44238 - t44239 + t47791 + t48241 + t44242 - t48242 + t44243 + t44245;
    let t48248 = t42496 - t47074 + t42501 + t42503 + t42506 - t47075 + t47078 + t42509 + t224 * (t47089 + t47092 + t47095 + t47098 + t47109 + t47124 + t47789 + t48243) + t47108 + t47110 - t42513;
    (t48248,)
}
