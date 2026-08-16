//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 282/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk282(t895: f64, t903: f64, t904: f64, t912: f64, t332: f64, t589: f64, t139: f64, t215: f64, t334: f64, t333: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t914 = t895 * t903 * t904;
    let t916 = 0.5848223622634646207e0_f64 * t912 * t914;
    let t917 = t589 * t332;
    let t921 = t215 * t139 * t334;
    let t923 = t333 * t921 / 288.0_f64;
    let t924 = t332 * t214;
    (t914, t916, t917, t921, t923, t924)
}
