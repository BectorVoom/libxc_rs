//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1207/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1207(t1266: f64, t3696: f64, t3703: f64, t11558: f64, t34337: f64, t11409: f64, t27286: f64, t11414: f64, t26996: f64, t11413: f64, t27290: f64, t563: f64) -> (f64, f64, f64, f64, f64) {
    let t34905 = t1266 * t3696 * t3703;
    let t34907 = t34337 * t11558;
    let t34909 = t11409 * t27286;
    let t34911 = t11414 * t26996;
    let t34914 = t563 * t11413 * t27290;
    (t34905, t34907, t34909, t34911, t34914)
}
