//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1078/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1078(t3114: f64, t4630: f64, t248: f64, t3101: f64, t4650: f64, t1020: f64, t10508: f64, t1616: f64, t122: f64, t247: f64) -> (f64, f64, f64, f64) {
    let t13959 = t3114 * t4630 / 2304.0_f64;
    let t13961 = t248 * t3101 * t4650;
    let t13963 = t1020 * t13961 / 2304.0_f64;
    let t13965 = t248 * t10508 * t1616;
    let t13966 = t1020 * t13965;
    let t13969 = t247 * t122;
    (t13959, t13963, t13966, t13969)
}
