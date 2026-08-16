//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1151/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1151(t1083: f64, t1085: f64, t1087: f64, t1089: f64, t2412: f64, t2958: f64, t3390: f64, t3394: f64, t3398: f64, t3402: f64, t3652: f64, t3656: f64, t3660: f64, t3664: f64, t9709: f64, t9711: f64, t9715: f64) -> f64 {
    let t42709 = 0.734774460522e2_f64 * t3390 * t2958 - 0.11494261417236e3_f64 * t3394 * t2958 + 0.6202613620464e2_f64 * t3398 * t2958 - 0.1088826475632e2_f64 * t3402 * t2958 + 0.1469548921044e3_f64 * t3652 * t2412 + 0.734774460522e2_f64 * t1083 * t9711 - 0.22988522834472e3_f64 * t3656 * t2412 - 0.11494261417236e3_f64 * t1085 * t9711 + 0.12405227240928e3_f64 * t3660 * t2412 + 0.6202613620464e2_f64 * t1087 * t9711 - 0.2177652951264e2_f64 * t3664 * t2412 - 0.1088826475632e2_f64 * t1089 * t9711 + 0.1469548921044e3_f64 * t1083 * t9709 - 0.22988522834472e3_f64 * t1083 * t9715 - 0.22988522834472e3_f64 * t1085 * t9709;
    t42709
}
