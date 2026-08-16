//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1246/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1246(t1020: f64, t1127: f64, t11274: f64, t11282: f64, t1129: f64, t1131: f64, t12298: f64, t1310: f64, t1312: f64, t2410: f64, t2412: f64, t339: f64, t341: f64, t343: f64, t3522: f64, t3526: f64, t3530: f64, t3745: f64, t3749: f64, t41978: f64, t839: f64, t8438: f64, t8465: f64) -> f64 {
    let t42035 = -0.18428227254588e2_f64 * t12298 * t839 - 0.9214113627294e1_f64 * t3749 * t1310 + 0.367387230261e2_f64 * t11282 * t1020 + 0.734774460522e2_f64 * t3530 * t2410 + 0.367387230261e2_f64 * t1131 * t8438 - 0.957855118103e1_f64 * t339 * t41978 + 0.3101306810232e1_f64 * t341 * t41978 - 0.362942158544e0_f64 * t343 * t41978 - 0.9214113627294e1_f64 * t1312 * t3745 - 0.8704e0_f64 * t8438 * t1127 - 0.17408e1_f64 * t2410 * t3522 - 0.8704e0_f64 * t1020 * t11274 - 0.8704e0_f64 * t1310 * t3745 + 0.1469548921044e3_f64 * t3526 * t2412 + 0.1469548921044e3_f64 * t1129 * t8465;
    t42035
}
