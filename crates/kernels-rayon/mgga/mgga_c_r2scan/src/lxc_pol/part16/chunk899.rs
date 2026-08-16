//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 899/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk899(t2924: f64, t818: f64, t2928: f64, t826: f64, t1010: f64, t2391: f64, t2938: f64, t1217: f64, t2358: f64, t2368: f64, t313: f64, t6678: f64, t9598: f64, t9608: f64, t9613: f64, t9623: f64, t9631: f64, t9635: f64) -> (f64, f64, f64, f64, f64) {
    let t9640 = t2924 * t818;
    let t9650 = t2928 * t826;
    let t9653 = t1010 * t2391;
    let t9657 = t2938 * t826;
    let t9673 = 3.0_f64 / 10.0_f64 * t313 * (-10.0_f64 / 27.0_f64 * t9598 + 20.0_f64 / 9.0_f64 * t2358 * t1217 + 10.0_f64 / 9.0_f64 * t9608 + 5.0_f64 / 3.0_f64 * t9613 - 10.0_f64 / 27.0_f64 * t9623 - 20.0_f64 / 9.0_f64 * t2368 * t1217 + 10.0_f64 / 9.0_f64 * t9631 + 5.0_f64 / 3.0_f64 * t9635) - t6678;
    (t9640, t9650, t9653, t9657, t9673)
}
