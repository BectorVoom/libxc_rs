//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 959/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk959<F: Float>(t44295: F, t4820: F, t6824: F, t13387: F, t4379: F, t11218: F, t1429: F, t2365: F, t2366: F, t11430: F, t2389: F, t13322: F, t13409: F, t13449: F, t13457: F, t1589: F, t1628: F, t40076: F, t41731: F, t46212: F, t46216: F, t46220: F, t46223: F, t46225: F, t46229: F, t46233: F, t46235: F, t46237: F, t46240: F, t4950: F, t557: F, t574: F, t597: F) -> F {
    let t46244 = F::cast_from(0.79445533226334281487e-1_f64) * t6824 * t4820 * t44295;
    let t46245 = t4379 * t13387;
    let t46246 = F::cast_from(0.14896037479937677779e-1_f64) * t46245;
    let t46249 = t1429 * t2365 * t2366 * t11218;
    let t46250 = F::cast_from(0.14896037479937677779e-1_f64) * t46249;
    let t46251 = t11430 * t2389;
    let t46252 = F::cast_from(0.29792074959875355558e-1_f64) * t46251;
    let t46253 = F::cast_from(0.11916829983950142223e0_f64) * t41731 - F::cast_from(0.23833659967900284446e0_f64) * t557 * t1589 * t13322 - F::cast_from(0.30674340763136599741e1_f64) * t574 * t1628 * t13457 + F::cast_from(0.30674340763136599741e1_f64) * t597 * t1628 * t13449 - F::cast_from(0.2556195063594716645e0_f64) * t40076 + t46212 + F::cast_from(0.14300195980740170668e1_f64) * t4950 * t13409 + t46216 + t46220 - t46223 - t46225 - t46229 - t46233 - F::cast_from(0.89376224879626066676e-1_f64) * t46235 + F::cast_from(0.44688112439813033338e-1_f64) * t46237 + F::cast_from(0.51123901271894332903e0_f64) * t46240 - t46244 - t46246 - t46250 - t46252;
    t46253
}
