//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1283/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1283(t11569: f64, t659: f64, t11558: f64, t35175: f64, t11408: f64, t27889: f64, t563: f64, t11405: f64, t3085: f64, t11342: f64, t11344: f64, t561: f64) -> (f64, f64, f64, f64, f64) {
    let t35302 = t11569 * t659;
    let t35304 = t35175 * t11558;
    let t35307 = t563 * t11408 * t27889;
    let t35309 = t11405 * t3085;
    let t35312 = t561 * t11342 * t11344;
    (t35302, t35304, t35307, t35309, t35312)
}
