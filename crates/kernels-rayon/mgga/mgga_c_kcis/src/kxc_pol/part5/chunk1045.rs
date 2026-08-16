//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1045/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1045(t1444: f64, t1477: f64, t1482: f64, t1409: f64, t3786: f64, t1319: f64, t1961: f64, t1996: f64, t3251: f64, t3255: f64, t5495: f64, t5500: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16369 = t1477 * t1444;
    let t16373 = t1482 * t1444;
    let t16387 = t3786 * t1409;
    let t16388 = t1961 * t1319;
    let t16401 = t3251 * t1996;
    let t16408 = 0.19711289e-2_f64 * t3255 * t5495;
    let t16410 = 0.26281718666666666666e-2_f64 * t3255 * t5500;
    (t16369, t16373, t16387, t16388, t16401, t16408, t16410)
}
