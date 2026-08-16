//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1247/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1247(t1129: f64, t1131: f64, t1133: f64, t1135: f64, t12300: f64, t1310: f64, t2412: f64, t3530: f64, t3534: f64, t3538: f64, t3753: f64, t41978: f64, t839: f64, t8440: f64, t8463: f64, t8465: f64) -> f64 {
    let t42067 = -0.22988522834472e3_f64 * t3530 * t2412 - 0.22988522834472e3_f64 * t1131 * t8465 + 0.12405227240928e3_f64 * t3534 * t2412 + 0.12405227240928e3_f64 * t1133 * t8465 - 0.2177652951264e2_f64 * t3538 * t2412 - 0.2177652951264e2_f64 * t1135 * t8465 + 0.734774460522e2_f64 * t1129 * t8463 - 0.11494261417236e3_f64 * t1131 * t8463 + 0.6202613620464e2_f64 * t1133 * t8463 - 0.1088826475632e2_f64 * t1135 * t8463 - 0.22988522834472e3_f64 * t1129 * t8440 + 0.18607840861392e3_f64 * t1131 * t8440 - 0.4355305902528e2_f64 * t1133 * t8440 + 0.734774460522e2_f64 * t12300 * t839 + 0.367387230261e2_f64 * t3753 * t1310 - 0.64e0_f64 * t41978;
    t42067
}
