//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 492/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk492(t1851: f64, t1853: f64, t547: f64, t548: f64, t10: f64, t17: f64, t551: f64, t555: f64, t15: f64, t22: f64, t11: f64, t14: f64) -> (f64, f64, f64, f64, f64) {
    let t1856 = t1851 * t548 + 3.0_f64 * t1853 * t547;
    let t1953 = 2.0_f64 * t10 * t17;
    let t1955 = 8.0_f64 * t551 * t555;
    let t1957 = 6.0_f64 * t15 * t22;
    let t1958 = t11 * t14;
    (t1856, t1953, t1955, t1957, t1958)
}
