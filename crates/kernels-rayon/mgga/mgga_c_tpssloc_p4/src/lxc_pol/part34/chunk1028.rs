//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1028/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1028(t22674: f64, t7691: f64, t22892: f64, t6883: f64, t7701: f64, t254: f64, t563: f64, t1827: f64, t22765: f64, t5234: f64, t6944: f64, t1824: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26197 = t22674 * t7691;
    let t26198 = t22892 * t26197;
    let t26200 = t6883 * t7701;
    let t26224 = t563 * t254;
    let t26231 = t22765 * t1827;
    let t26233 = t5234 * t6944;
    let t26243 = t236 * t1824;
    (t26197, t26198, t26200, t26224, t26231, t26233, t26243)
}
