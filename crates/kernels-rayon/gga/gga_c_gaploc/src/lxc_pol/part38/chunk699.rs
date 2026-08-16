//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 699/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk699(t13415: f64, t1572: f64, t12940: f64, t12944: f64, t12946: f64, t13385: f64, t13389: f64, t13390: f64, t13395: f64, t13399: f64, t13405: f64, t13409: f64, t13412: f64, t597: f64) -> f64 {
    let t13417 = 0.71500979903700853338e0_f64 * t1572 * t13415;
    let t13418 = t13385 - t13389 - 0.44688112439813033338e-1_f64 * t13390 + t13395 + 0.95857314884801874192e0_f64 * t13399 - t13405 - 0.63904876589867916128e-1_f64 * t12940 - 0.59584149919750711116e-1_f64 * t12944 + 0.59584149919750711116e-1_f64 * t12946 + 0.14300195980740170668e1_f64 * t1572 * t13409 + 0.23005755572352449806e2_f64 * t597 * t13412 + t13417;
    t13418
}
