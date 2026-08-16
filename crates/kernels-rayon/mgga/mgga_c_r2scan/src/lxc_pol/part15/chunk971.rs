//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 971/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk971(t11056: f64, t819: f64, t11032: f64, t11035: f64, t11037: f64, t11039: f64, t11041: f64, t11043: f64, t11046: f64, t11048: f64, t11052: f64, t11054: f64) -> (f64, f64) {
    let t11057 = t819 * t11056;
    let t11058 = 11.0_f64 / 9.0_f64 * t11057;
    let t11059 = -t11032 - t11035 - t11037 / 4.0_f64 + t11039 / 8.0_f64 - t11041 / 8.0_f64 + t11043 / 2.0_f64 + t11046 - 3.0_f64 / 4.0_f64 * t11048 - t11052 + t11054 / 4.0_f64 - t11058;
    (t11058, t11059)
}
