//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 362/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk362(t1647: f64, t1649: f64, t563: f64, t589: f64, t505: f64, t597: f64, t599: f64, t561: f64, t595: f64, t198: f64, t672: f64, t674: f64, t681: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1650 = t1647 * t1649;
    let t1653 = t563 * t589;
    let t1659 = t597 * t505 * t599;
    let t1662 = t561 * t595;
    let t1665 = t672 * t198;
    let t1666 = t674 * t681;
    (t1650, t1653, t1659, t1662, t1665, t1666)
}
