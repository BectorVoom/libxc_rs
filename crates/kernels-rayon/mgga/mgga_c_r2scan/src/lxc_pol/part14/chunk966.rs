//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 966/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk966(t1127: f64, t1310: f64, t3522: f64, t839: f64, t11274: f64, t333: f64, t335: f64, t337: f64, t339: f64, t341: f64, t1131: f64, t1133: f64, t1135: f64, t1312: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11276 = t1310 * t1127;
    let t11278 = t839 * t3522;
    let t11280 = t333 * t11274;
    let t11282 = t335 * t11274;
    let t11284 = t337 * t11274;
    let t11286 = t339 * t11274;
    let t11288 = t341 * t11274;
    let t11298 = -0.64e0_f64 * t11274 - 0.8704e0_f64 * t11276 - 0.17408e1_f64 * t11278 - 0.8704e0_f64 * t11280 - 0.4607056813647e1_f64 * t11282 + 0.122462410087e2_f64 * t11284 - 0.957855118103e1_f64 * t11286 + 0.3101306810232e1_f64 * t11288 - 0.362942158544e0_f64 * t343 * t11274 - 0.11494261417236e3_f64 * t1131 * t1312 + 0.6202613620464e2_f64 * t1133 * t1312 - 0.1088826475632e2_f64 * t1135 * t1312;
    (t11276, t11278, t11280, t11282, t11284, t11286, t11288, t11298)
}
