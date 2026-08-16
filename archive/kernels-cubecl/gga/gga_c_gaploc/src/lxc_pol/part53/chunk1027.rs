//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1027/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1027<F: Float>(t42267: F, t42272: F, t42275: F, t42278: F, t42282: F, t42284: F, t42288: F, t42292: F, t42298: F, t42305: F, t42309: F, t42312: F, t42315: F, t42340: F, t48121: F, t48124: F, t48127: F, t48131: F, t48134: F, t48137: F) -> F {
    let t50902 = t42267 + t42272 + t42275 + t42278 + F::cast_from(0.51123901271894332901e0_f64) * t48121 + F::cast_from(0.14300195980740170668e1_f64) * t48124 + t42282 - t42284 - t42288 - t42292 - t42298 + F::cast_from(0.95334639871601137787e0_f64) * t48127 + t42305 - t42309 - t42312 + F::cast_from(0.23005755572352449806e2_f64) * t48131 + F::cast_from(0.23005755572352449806e2_f64) * t48134 + F::cast_from(0.23005755572352449806e2_f64) * t48137 - t42315 + t42340;
    t50902
}
