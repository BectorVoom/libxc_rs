//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 388/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk388<F: Float>(t1238: F, t1241: F, t1243: F, t1247: F, t1249: F, t1251: F, t404: F, t389: F) -> (F, F, F) {
    let t1285 = -F::cast_from(0.42198333333333333333e0_f64) * t1238 + F::cast_from(0.84396666666666666666e0_f64) * t1241 + F::cast_from(0.39862222222222222223e0_f64) * t1243 + F::cast_from(0.68258333333333333333e-1_f64) * t1247 + F::cast_from(0.13651666666666666667e0_f64) * t1249 + F::cast_from(0.13692777777777777778e0_f64) * t1251;
    let t1286 = t1285 * t404;
    let t1287 = t389 * t1286;
    let t1288 = F::cast_from(1.0_f64) * t1287;
    (t1285, t1286, t1288)
}
