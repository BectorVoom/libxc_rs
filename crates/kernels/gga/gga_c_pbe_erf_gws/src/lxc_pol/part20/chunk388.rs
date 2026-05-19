//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 388/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk388<F: Float>(t1238: F, t1241: F, t1243: F, t1247: F, t1249: F, t1251: F, t441: F, t433: F, t62: F, t70: F, t1231: F, t31: F, t4: F, t542: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1253 = -F::cast_from(0.78438333333333333333e0_f64) * t1238 + F::cast_from(0.15687666666666666667e1_f64) * t1241 + F::cast_from(0.68863333333333333333e0_f64) * t1243 + F::cast_from(0.14025833333333333333e0_f64) * t1247 + F::cast_from(0.28051666666666666667e0_f64) * t1249 + F::cast_from(0.17365833333333333333e0_f64) * t1251;
    let t1254 = t1253 * t441;
    let t1257 = t433 * t433;
    let t1258 = F::new(1.0) / t1257;
    let t1259 = t62 * t1258;
    let t1260 = t70 * t70;
    let t1261 = F::new(1.0) / t1260;
    let t1262 = t1231 * t1261;
    let t1266 = t4 * t542 * t31;
    (t1253, t1254, t1257, t1258, t1259, t1260, t1261, t1262, t1266)
}
