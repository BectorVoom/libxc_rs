//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 952/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk952(t42378: f64, t40514: f64, t40517: f64, t9065: f64, t986: f64, t1415: f64, t1646: f64, t42148: f64, t4446: f64, t10547: f64, t9333: f64, t12868: f64, t1580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42379 = 0.63904876589867916128e-1_f64 * t42378;
    let t42380 = 0.59584149919750711116e-1_f64 * t40514;
    let t42381 = 0.25561950635947166451e0_f64 * t40517;
    let t42382 = t9065 * t986;
    let t42385 = 0.35750489951850426669e0_f64 * t1415 * t42382 * t1646;
    let t42388 = 0.25025342966295298669e1_f64 * t1415 * t42148 * t4446;
    let t42390 = 0.50050685932590597338e1_f64 * t10547 * t9333;
    let t42392 = 0.11502877786176224903e2_f64 * t1580 * t12868;
    (t42379, t42380, t42381, t42385, t42388, t42390, t42392)
}
