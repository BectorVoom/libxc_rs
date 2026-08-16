//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 662/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk662(t640: f64, t667: f64, t1870: f64, t618: f64, t5: f64, t515: f64, t1784: f64, t203: f64, t674: f64, t1946: f64, t350: f64, t1457: f64, t563: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5463 = t640 * t667;
    let t5479 = t618 * t1870;
    let t5486 = t515 * t5;
    let t5510 = t1784 * t674 * t203;
    let t5526 = t1946 * t350;
    let t5541 = t563 * t1457;
    (t5463, t5479, t5486, t5510, t5526, t5541)
}
