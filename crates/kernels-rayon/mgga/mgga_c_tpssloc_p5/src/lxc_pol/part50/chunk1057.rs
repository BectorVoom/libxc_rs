//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1057/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1057(t31043: f64, t8493: f64, t1983: f64, t532: f64, t8488: f64, t6879: f64, t1976: f64, t6534: f64, t652: f64, t2314: f64, t8327: f64, t4034: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31044 = t8493 * t31043;
    let t31046 = 2.0_f64 * t1983 * t31044;
    let t31047 = t532 * t8488;
    let t31048 = t31047 * t6879;
    let t31050 = 3.0_f64 * t1983 * t31048;
    let t31051 = t1976 * t6534;
    let t31052 = t652 * t31051;
    let t31054 = t2314 * t8327;
    let t31055 = 2.0_f64 * t31054;
    let t31056 = t4034 * t8327;
    (t31044, t31046, t31047, t31048, t31050, t31051, t31052, t31055, t31056)
}
