//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1242/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1242(t1020: f64, t1087: f64, t1089: f64, t1091: f64, t11092: f64, t11118: f64, t11120: f64, t1310: f64, t1312: f64, t2410: f64, t3402: f64, t3406: f64, t3652: f64, t3656: f64, t3660: f64, t3664: f64, t3668: f64, t8438: f64, t8440: f64) -> f64 {
    let t40954 = -0.4355305902528e2_f64 * t1087 * t8440 + 0.6202613620464e2_f64 * t3660 * t1312 - 0.1088826475632e2_f64 * t3664 * t1312 + 0.734774460522e2_f64 * t11092 * t1020 + 0.734774460522e2_f64 * t3652 * t1312 - 0.11494261417236e3_f64 * t3656 * t1312 - 0.3831420472412e2_f64 * t3660 * t1310 + 0.1550653405116e2_f64 * t11118 * t1020 + 0.3101306810232e2_f64 * t3402 * t2410 + 0.1550653405116e2_f64 * t1089 * t8438 + 0.1550653405116e2_f64 * t3664 * t1310 - 0.2177652951264e1_f64 * t11120 * t1020 - 0.4355305902528e1_f64 * t3406 * t2410 - 0.2177652951264e1_f64 * t1091 * t8438 - 0.2177652951264e1_f64 * t3668 * t1310;
    t40954
}
