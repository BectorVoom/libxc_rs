//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 387/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk387(t506: f64, t583: f64, t181: f64, t1338: f64, t172: f64, t153: f64, t515: f64, t563: f64) -> (f64, f64, f64, f64) {
    let t1866 = t506 * t583;
    let t1867 = t181 * t1866;
    let t1870 = t172 * t1338;
    let t1871 = t153 * t1870;
    let t1872 = t181 * t1871;
    let t1875 = t563 * t515;
    (t1867, t1870, t1872, t1875)
}
