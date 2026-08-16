//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 323/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk323<F: Float>(t1303: F, t73: F, t405: F, t457: F, t1338: F, t115: F, t1249: F, t1283: F, t1287: F, t1294: F, t1304: F, t1339: F, t1386: F, t1388: F, t1393: F, t154: F, t155: F, t169: F, t528: F, t532: F, t536: F, t561: F, t563: F, t564: F) -> F {
    let t1394 = t73 * t1303;
    let t1397 = t405 * t457;
    let t1400 = t73 * t1338;
    let t1403 = -F::cast_from(0.17687407407407407407e-1_f64) * t154 * t1283 * t115 + F::cast_from(0.10612444444444444444e0_f64) * t154 * t1287 * t115 - F::cast_from(0.10612444444444444444e0_f64) * t154 * t528 * t536 + F::cast_from(0.79593333333333333331e-1_f64) * t154 * t1294 * t115 - F::cast_from(0.15918666666666666666e0_f64) * t154 * t532 * t536 + F::cast_from(0.15918666666666666666e0_f64) * t154 * t155 * t1304 - F::cast_from(0.79593333333333333331e-1_f64) * t154 * t155 * t1339 - t1386 * t73 + F::cast_from(2.0_f64) * t1388 * t564 - F::cast_from(2.0_f64) * t561 * t405 - F::cast_from(2.0_f64) * t1393 * t1394 + F::cast_from(2.0_f64) * t563 * t1397 + t563 * t1400 - t169 * t1249;
    t1403
}
