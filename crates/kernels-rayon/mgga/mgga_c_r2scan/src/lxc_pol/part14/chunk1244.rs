//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1244/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1244(t1020: f64, t11284: f64, t11286: f64, t11288: f64, t1133: f64, t1135: f64, t1137: f64, t1310: f64, t1312: f64, t2410: f64, t3534: f64, t3538: f64, t3542: f64, t3749: f64, t3753: f64, t3757: f64, t3761: f64, t3765: f64, t8438: f64) -> f64 {
    let t41971 = -0.3831420472412e2_f64 * t11284 * t1020 - 0.7662840944824e2_f64 * t3534 * t2410 - 0.3831420472412e2_f64 * t1133 * t8438 - 0.3831420472412e2_f64 * t3757 * t1310 + 0.1550653405116e2_f64 * t11286 * t1020 + 0.3101306810232e2_f64 * t3538 * t2410 + 0.1550653405116e2_f64 * t1135 * t8438 + 0.1550653405116e2_f64 * t3761 * t1310 - 0.2177652951264e1_f64 * t11288 * t1020 - 0.4355305902528e1_f64 * t3542 * t2410 - 0.2177652951264e1_f64 * t1137 * t8438 - 0.2177652951264e1_f64 * t3765 * t1310 + 0.734774460522e2_f64 * t3749 * t1312 - 0.11494261417236e3_f64 * t3753 * t1312 + 0.6202613620464e2_f64 * t3757 * t1312;
    t41971
}
