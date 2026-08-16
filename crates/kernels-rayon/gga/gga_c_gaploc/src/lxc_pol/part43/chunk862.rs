//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 862/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk862(t34400: f64, t34401: f64, t41726: f64, t12938: f64, t2464: f64, t587: f64, t40514: f64, t40517: f64, t9065: f64, t986: f64, t1415: f64, t1646: f64) -> (f64, f64, f64, f64, f64) {
    let t42376 = 0.13803453343411469884e3_f64 * t34400 * t34401 * t41726;
    let t42378 = t587 * t2464 * t12938;
    let t42379 = 0.63904876589867916128e-1_f64 * t42378;
    let t42380 = 0.59584149919750711116e-1_f64 * t40514;
    let t42381 = 0.25561950635947166451e0_f64 * t40517;
    let t42382 = t9065 * t986;
    let t42385 = 0.35750489951850426669e0_f64 * t1415 * t42382 * t1646;
    (t42376, t42379, t42380, t42381, t42385)
}
