//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 163/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk163(t457: f64, t73: f64, t115: f64, t154: f64, t155: f64, t169: f64, t405: f64, t528: f64, t532: f64, t536: f64, t561: f64, t563: f64) -> (f64, f64) {
    let t564 = t73 * t457;
    let t567 = 0.53062222222222222221e-1_f64 * t154 * t528 * t115 + 0.79593333333333333331e-1_f64 * t154 * t532 * t115 - 0.79593333333333333331e-1_f64 * t154 * t155 * t536 - t561 * t73 + t563 * t564 - t169 * t405;
    (t564, t567)
}
