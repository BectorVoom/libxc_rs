//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1185/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1185(t1020: f64, t3396: f64, t568: f64, t16193: f64, t16230: f64, t16273: f64, t16275: f64, t16280: f64, t16283: f64, t16287: f64, t16290: f64, t19624: f64, t19688: f64, t19690: f64, t28914: f64, t28916: f64, t28917: f64, t28918: f64, t28919: f64) -> (f64, f64, f64) {
    let t29093 = t1020 * t3396;
    let t29094 = t29093 * t568;
    let t29111 = -t16193 + t28914 + t28916 - t16230 - t16273 + t16275 - t28917 + t19624 + t28918 + t28919 - t16280 + t16283 + t16287 - t16290 + t19688 + t19690;
    (t29093, t29094, t29111)
}
