//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 349/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk349(t1312: f64, t333: f64, t335: f64, t337: f64, t339: f64, t1310: f64, t341: f64, t343: f64, t349: f64, t854: f64) -> (f64, f64, f64) {
    let t1316 = t333 * t1312;
    let t1320 = t335 * t1312;
    let t1324 = t337 * t1312;
    let t1328 = t339 * t1312;
    let t1336 = -0.64e0_f64 * t1310 - 0.8704e0_f64 * t1312 - 0.8704e0_f64 * t333 * t1310 - 0.9214113627294e1_f64 * t1316 - 0.4607056813647e1_f64 * t335 * t1310 + 0.367387230261e2_f64 * t1320 + 0.122462410087e2_f64 * t337 * t1310 - 0.3831420472412e2_f64 * t1324 - 0.957855118103e1_f64 * t339 * t1310 + 0.1550653405116e2_f64 * t1328 + 0.3101306810232e1_f64 * t341 * t1310 - 0.2177652951264e1_f64 * t341 * t1312 - 0.362942158544e0_f64 * t343 * t1310;
    let t1337 = t854 * t349;
    let t1338 = 1.0_f64 / t1337;
    (t1336, t1337, t1338)
}
