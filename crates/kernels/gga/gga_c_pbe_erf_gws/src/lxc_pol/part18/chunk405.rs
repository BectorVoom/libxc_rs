//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 405/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk405<F: Float>(t1383: F, t169: F, t289: F, t274: F, t39: F, t532: F, t745: F, t1216: F, t1319: F, t1322: F, t470: F, t1314: F, t449: F, t456: F) -> (F, F, F, F, F, F) {
    let t1386 = F::cast_from(0.31835665774679373271e-1_f64) * t169 * t289 * t1383;
    let t1388 = F::cast_from(0.3199504064530762818e0_f64) * t39 * t274;
    let t1389 = t532 * t745;
    let t1392 = t1319 * t1216 * t1322;
    let t1393 = t470 * t1392;
    let t1394 = F::cast_from(0.17315755899375863299e2_f64) * t1393;
    let t1396 = t449 * t1314 * t456;
    (t1386, t1388, t1389, t1392, t1394, t1396)
}
