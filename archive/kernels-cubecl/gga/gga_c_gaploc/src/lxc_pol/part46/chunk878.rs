//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 878/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk878<F: Float>(t1445: F, t42009: F, t42250: F, t42254: F, t42257: F, t42259: F, t42263: F, t42265: F, t42267: F, t42269: F, t42272: F, t42275: F, t42278: F, t42279: F, t42282: F, t42284: F, t42288: F, t42292: F, t42298: F, t42299: F, t42305: F, t574: F, t597: F) -> F {
    let t42306 = t42250 + F::cast_from(0.85801175884441024008e1_f64) * t42254 + t42257 + F::cast_from(0.29792074959875355558e-1_f64) * t42259 - t42263 + t42265 + t42267 + F::cast_from(0.14300195980740170668e1_f64) * t42269 + t42272 + t42275 + t42278 + F::cast_from(0.14300195980740170668e1_f64) * t42279 + t42282 - t42284 - t42288 - t42292 + F::cast_from(0.11502877786176224903e2_f64) * t597 * t1445 * t42009 - t42298 - F::cast_from(0.46011511144704899612e1_f64) * t574 * t1445 * t42299 + t42305;
    t42306
}
