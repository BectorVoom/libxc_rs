//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 877/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk877(t15: f64, t563: f64, t11: f64, t2: f64, t22: f64, t1958: f64, t27: f64, t559: f64, t20: f64, t571: f64, t12: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7656 = 24.0_f64 * t15 * t563;
    let t7657 = t11 * t2;
    let t7659 = 24.0_f64 * t7657 * t22;
    let t7660 = t1958 * t563;
    let t7662 = t559 * t27;
    let t7665 = 120.0_f64 * t20 * t571;
    let t7666 = t12 * t558;
    (t7656, t7659, t7660, t7662, t7665, t7666)
}
