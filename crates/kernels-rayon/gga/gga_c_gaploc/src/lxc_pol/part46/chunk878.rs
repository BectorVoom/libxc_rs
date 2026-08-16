//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 878/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk878(t1445: f64, t42009: f64, t42250: f64, t42254: f64, t42257: f64, t42259: f64, t42263: f64, t42265: f64, t42267: f64, t42269: f64, t42272: f64, t42275: f64, t42278: f64, t42279: f64, t42282: f64, t42284: f64, t42288: f64, t42292: f64, t42298: f64, t42299: f64, t42305: f64, t574: f64, t597: f64) -> f64 {
    let t42306 = t42250 + 0.85801175884441024008e1_f64 * t42254 + t42257 + 0.29792074959875355558e-1_f64 * t42259 - t42263 + t42265 + t42267 + 0.14300195980740170668e1_f64 * t42269 + t42272 + t42275 + t42278 + 0.14300195980740170668e1_f64 * t42279 + t42282 - t42284 - t42288 - t42292 + 0.11502877786176224903e2_f64 * t597 * t1445 * t42009 - t42298 - 0.46011511144704899612e1_f64 * t574 * t1445 * t42299 + t42305;
    t42306
}
