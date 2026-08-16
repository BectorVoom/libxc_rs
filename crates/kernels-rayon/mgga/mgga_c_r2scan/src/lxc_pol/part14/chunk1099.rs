//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1099/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1099(t11310: f64, t1338: f64, t3552: f64, t6755: f64, t1142: f64, t19309: f64, t1348: f64, t6767: f64, t19327: f64, t1114: f64, t23040: f64, t3493: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38953 = t1338 * t11310;
    let t38958 = t6755 * t3552;
    let t38961 = t19309 * t1142;
    let t38966 = t1348 * t11310;
    let t38971 = t6767 * t3552;
    let t38976 = t19327 * t1142;
    let t39010 = t23040 * t1114;
    let t39014 = t3493 * t481;
    (t38953, t38958, t38961, t38966, t38971, t38976, t39010, t39014)
}
