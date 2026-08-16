//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 956/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk956(t46189: f64, t13398: f64, t7014: f64, t11172: f64, t2464: f64, t2465: f64, t2487: f64, t13322: f64, t13412: f64, t1441: f64, t1580: f64, t188: f64, t189: f64, t193: f64, t2386: f64, t40009: f64, t40013: f64, t40015: f64, t40019: f64, t41705: f64, t41711: f64, t44601: f64, t46167: f64, t46168: f64, t46169: f64, t46170: f64, t46174: f64, t46175: f64, t46176: f64, t46181: f64, t590: f64) -> f64 {
    let t46190 = 0.14896037479937677779e-1_f64 * t46189;
    let t46191 = t7014 * t13398;
    let t46195 = t2487 * t2464 * t2465 * t11172;
    let t46197 = 0.35750489951850426669e0_f64 * t188 * t189 * t44601 * t193 + t46167 - t46168 - t46169 + t46170 + 0.51123901271894332902e0_f64 * t1441 * t13322 * t590 - t46174 - t46175 + t46176 + 0.23005755572352449806e2_f64 * t1580 * t13412 - 0.63904876589867916128e-1_f64 * t41705 - 0.63904876589867916128e-1_f64 * t41711 - 0.10725146985555128001e1_f64 * t46181 * t2386 - 0.63904876589867916126e-1_f64 * t40009 - 0.63904876589867916126e-1_f64 * t40013 + 0.63904876589867916126e-1_f64 * t40015 + 0.63904876589867916126e-1_f64 * t40019 + t46190 + 0.95857314884801874192e0_f64 * t46191 - 0.21301625529955972043e0_f64 * t46195;
    t46197
}
