//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 859/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk859(t7612: f64, t2: f64, t22: f64, t106: f64, t107: f64, t10: f64, t555: f64, t551: f64, t15: f64, t563: f64, t11: f64, t1958: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7613 = 1.0_f64 / t7612;
    let t7622 = t2 * t22;
    let t7628 = t107 * t106;
    let t7629 = 1.0_f64 / t7628;
    let t7651 = t10 * t555;
    let t7653 = t551 * t22;
    let t7656 = 24.0_f64 * t15 * t563;
    let t7657 = t11 * t2;
    let t7659 = 24.0_f64 * t7657 * t22;
    let t7660 = t1958 * t563;
    (t7613, t7622, t7629, t7651, t7653, t7656, t7659, t7660)
}
