//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1012/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1012(t19509: f64, t2981: f64, t19508: f64, t1649: f64, t5391: f64, t137: f64, t1552: f64, t442: f64, t5964: f64, t4048: f64, t563: f64, t1577: f64, t19422: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19510 = t19509 * t2981;
    let t19511 = t19508 * t19510;
    let t19522 = t1649 * t5391;
    let t19530 = t5964 * t1552 * t137 * t442;
    let t19533 = t563 * t4048;
    let t19535 = t19422 * t1577;
    (t19510, t19511, t19522, t19530, t19533, t19535)
}
