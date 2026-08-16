//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 179/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk179(t11: f64, t558: f64, t22: f64, t21: f64, t3: f64) -> (f64, f64, f64, f64) {
    let t559 = t11 * t558;
    let t561 = 4.0_f64 * t559 * t22;
    let t562 = t21 * t3;
    let t563 = 1.0_f64 / t562;
    (t559, t561, t562, t563)
}
