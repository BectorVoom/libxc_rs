//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 420/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk420(t1914: f64, t202: f64, t193: f64, t870: f64, t1915: f64, t28: f64, t1877: f64, t1268: f64, t1873: f64, t191: f64, t513: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1962 = t202 * t1914;
    let t1964 = t193 * t1962 * t870;
    let t1969 = t1915 * t28;
    let t1971 = t1877 * t1969 / 2.0_f64;
    let t1979 = 2.0_f64 * t1268 * t1873;
    let t1982 = t513 * t191;
    let t1983 = t1982 * t192;
    (t1962, t1964, t1969, t1971, t1979, t1982, t1983)
}
