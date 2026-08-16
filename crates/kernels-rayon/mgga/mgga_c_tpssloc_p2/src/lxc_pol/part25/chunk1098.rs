//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1098/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1098(t1336: f64, t2690: f64, t6943: f64, t1354: f64, t1339: f64, t55003: f64, t6936: f64, t22770: f64, t22779: f64, t22773: f64, t12178: f64, t12168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80914 = t1336 * t6943 * t2690;
    let t80915 = t80914 * t1354;
    let t80918 = t6936 * t1339 * t55003;
    let t80920 = t22779 * t22770;
    let t80922 = t22779 * t22773;
    let t80925 = t6936 * t1339 * t12178;
    let t80928 = t6936 * t1339 * t12168;
    (t80915, t80918, t80920, t80922, t80925, t80928)
}
