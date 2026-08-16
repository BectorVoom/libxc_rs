//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 493/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk493(t1958: f64, t22: f64, t559: f64, t563: f64, t20: f64, t27: f64, t12: f64, t19: f64, t567: f64, t571: f64, t21: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1960 = 12.0_f64 * t1958 * t22;
    let t1962 = 32.0_f64 * t559 * t563;
    let t1964 = 20.0_f64 * t20 * t27;
    let t1965 = t12 * t19;
    let t1967 = 30.0_f64 * t1965 * t27;
    let t1969 = 72.0_f64 * t567 * t571;
    let t1970 = t21 * t21;
    let t1971 = 1.0_f64 / t1970;
    let t1973 = 42.0_f64 * t25 * t1971;
    (t1960, t1962, t1964, t1965, t1967, t1969, t1970, t1971, t1973)
}
