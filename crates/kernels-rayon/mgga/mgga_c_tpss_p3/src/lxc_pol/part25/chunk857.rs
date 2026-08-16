//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 857/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk857(t1338: f64, t5953: f64, t117: f64, t6323: f64, t1668: f64, t1670: f64, t1851: f64, t1853: f64, t547: f64, t548: f64, t6446: f64, t562: f64, t65: f64) -> (f64, f64, f64, f64) {
    let t6452 = t5953 * t1338;
    let t6455 = t117 * t6323;
    let t6458 = 3.0_f64 * t1668 * t1853 + 3.0_f64 * t1670 * t1851 + 6.0_f64 * t547 * t6452 + 3.0_f64 * t547 * t6455 + t548 * t6446;
    let t7091 = 1.0_f64 / t65 / t562;
    (t6452, t6455, t6458, t7091)
}
