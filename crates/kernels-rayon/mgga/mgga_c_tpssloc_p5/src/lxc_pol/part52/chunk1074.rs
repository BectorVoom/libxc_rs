//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1074/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1074(t254: f64, t563: f64, t12020: f64, t2015: f64, t5325: f64, t1323: f64, t7722: f64, t1827: f64, t22765: f64, t5234: f64, t6944: f64, t1354: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26224 = t563 * t254;
    let t26225 = t12020 * t2015;
    let t26226 = t26225 * t5325;
    let t26229 = t1323 * t7722;
    let t26231 = t22765 * t1827;
    let t26233 = t5234 * t6944;
    let t26234 = t26233 * t1354;
    (t26224, t26225, t26226, t26229, t26231, t26234)
}
