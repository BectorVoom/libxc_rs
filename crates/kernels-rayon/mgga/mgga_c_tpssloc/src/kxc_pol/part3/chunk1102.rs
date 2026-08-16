//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1102/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1102(t2844: f64, t4395: f64, t912: f64, t2842: f64, t2836: f64, t4399: f64, t10704: f64, t1556: f64, t2793: f64, t10702: f64, t13566: f64, t13602: f64) -> (f64, f64, f64, f64, f64) {
    let t14388 = t4395 * t2844;
    let t14389 = t14388 * t912;
    let t14391 = 0.32163958997385070134e2_f64 * t2842 * t14389;
    let t14392 = t4399 * t2836;
    let t14394 = 0.16081979498692535067e2_f64 * t2842 * t14392;
    let t14395 = t1556 * t10704;
    let t14396 = t14395 * t2793;
    let t14398 = 0.51726012919273400301e3_f64 * t10702 * t14396;
    let t14409 = 0.2283111111111111111e-1_f64 * t13566;
    let t14410 = 0.11415555555555555555e-1_f64 * t13602;
    (t14391, t14394, t14398, t14409, t14410)
}
