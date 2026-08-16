//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 870/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk870(t1444: f64, t1961: f64, t2642: f64, t3766: f64, t1996: f64, t3251: f64, t3815: f64, t5804: f64, t5498: f64, t3255: f64, t5495: f64, t5500: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16397 = t1961 * t1444 * t2642;
    let t16398 = t3766 * t16397;
    let t16401 = t3251 * t1996;
    let t16403 = t5804 * t3815;
    let t16404 = t5498 * t16403;
    let t16408 = 0.19711289e-2_f64 * t3255 * t5495;
    let t16410 = 0.26281718666666666666e-2_f64 * t3255 * t5500;
    (t16398, t16401, t16403, t16404, t16408, t16410)
}
