//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1195/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1195(t11558: f64, t34337: f64, t11409: f64, t27286: f64, t11414: f64, t26996: f64, t11413: f64, t27290: f64, t563: f64, t144: f64, t21281: f64, t21283: f64, t33267: f64) -> (f64, f64, f64, f64, f64) {
    let t34907 = t34337 * t11558;
    let t34909 = t11409 * t27286;
    let t34911 = t11414 * t26996;
    let t34914 = t563 * t11413 * t27290;
    let t34918 = t21281 * t33267 * t144 * t21283;
    (t34907, t34909, t34911, t34914, t34918)
}
