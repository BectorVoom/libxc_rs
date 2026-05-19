//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 393/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk393<F: Float>(t387: F, t13: F, t30: F, t1275: F) -> (F, F, F, F, F, F, F) {
    let t1289 = t387 * t387;
    let t1290 = F::new(1.0) / t1289;
    let t1291 = t13 * t1290;
    let t1292 = t30 * t30;
    let t1293 = F::new(1.0) / t1292;
    let t1294 = t1275 * t1293;
    let t1295 = t1291 * t1294;
    let t1296 = F::cast_from(0.16081824322151104822e2_f64) * t1295;
    (t1289, t1290, t1291, t1292, t1293, t1294, t1296)
}
