//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 956/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk956<F: Float>(t46189: F, t13398: F, t7014: F, t11172: F, t2464: F, t2465: F, t2487: F, t13322: F, t13412: F, t1441: F, t1580: F, t188: F, t189: F, t193: F, t2386: F, t40009: F, t40013: F, t40015: F, t40019: F, t41705: F, t41711: F, t44601: F, t46167: F, t46168: F, t46169: F, t46170: F, t46174: F, t46175: F, t46176: F, t46181: F, t590: F) -> F {
    let t46190 = F::new(0.14896037479937677779e-1) * t46189;
    let t46191 = t7014 * t13398;
    let t46195 = t2487 * t2464 * t2465 * t11172;
    let t46197 = F::new(0.35750489951850426669e0) * t188 * t189 * t44601 * t193 + t46167 - t46168 - t46169 + t46170 + F::new(0.51123901271894332902e0) * t1441 * t13322 * t590 - t46174 - t46175 + t46176 + F::new(0.23005755572352449806e2) * t1580 * t13412 - F::new(0.63904876589867916128e-1) * t41705 - F::new(0.63904876589867916128e-1) * t41711 - F::new(0.10725146985555128001e1) * t46181 * t2386 - F::new(0.63904876589867916126e-1) * t40009 - F::new(0.63904876589867916126e-1) * t40013 + F::new(0.63904876589867916126e-1) * t40015 + F::new(0.63904876589867916126e-1) * t40019 + t46190 + F::new(0.95857314884801874192e0) * t46191 - F::new(0.21301625529955972043e0) * t46195;
    t46197
}
