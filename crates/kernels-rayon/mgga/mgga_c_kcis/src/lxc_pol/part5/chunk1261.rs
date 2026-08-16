//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1261/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1261(t1464: f64, t21047: f64, t16617: f64, t5875: f64, t1395: f64, t17298: f64, t5638: f64, t1307: f64, t7282: f64, t4162: f64, t4160: f64, t1365: f64, t7054: f64) -> (f64, f64, f64, f64, f64) {
    let t21048 = t1464 * t21047;
    let t21050 = t16617 * t5875;
    let t21051 = t1395 * t21050;
    let t21052 = t1464 * t21051;
    let t21055 = t17298 * t5638;
    let t21057 = t7282 * t1307;
    let t21058 = t4162 * t21057;
    let t21059 = t4160 * t21058;
    let t21061 = t7054 * t1365;
    (t21048, t21052, t21055, t21059, t21061)
}
