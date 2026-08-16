//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 323/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk323(t1303: f64, t73: f64, t405: f64, t457: f64, t1338: f64, t115: f64, t1249: f64, t1283: f64, t1287: f64, t1294: f64, t1304: f64, t1339: f64, t1386: f64, t1388: f64, t1393: f64, t154: f64, t155: f64, t169: f64, t528: f64, t532: f64, t536: f64, t561: f64, t563: f64, t564: f64) -> f64 {
    let t1394 = t73 * t1303;
    let t1397 = t405 * t457;
    let t1400 = t73 * t1338;
    let t1403 = -0.17687407407407407407e-1_f64 * t154 * t1283 * t115 + 0.10612444444444444444e0_f64 * t154 * t1287 * t115 - 0.10612444444444444444e0_f64 * t154 * t528 * t536 + 0.79593333333333333331e-1_f64 * t154 * t1294 * t115 - 0.15918666666666666666e0_f64 * t154 * t532 * t536 + 0.15918666666666666666e0_f64 * t154 * t155 * t1304 - 0.79593333333333333331e-1_f64 * t154 * t155 * t1339 - t1386 * t73 + 2.0_f64 * t1388 * t564 - 2.0_f64 * t561 * t405 - 2.0_f64 * t1393 * t1394 + 2.0_f64 * t563 * t1397 + t563 * t1400 - t169 * t1249;
    t1403
}
