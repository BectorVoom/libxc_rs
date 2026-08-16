//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1125/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1125(t14210: f64, t3293: f64, t1035: f64, t1670: f64, t3074: f64, t10314: f64, t1662: f64, t13495: f64, t4579: f64, t10324: f64, t2944: f64, t3255: f64, t4572: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14211 = t3293 * t14210;
    let t14215 = t1035 * t1670;
    let t14216 = t14215 * t3074;
    let t14217 = t3293 * t14216;
    let t14221 = t10314 * t1662 * t3074;
    let t14224 = t4579 * t13495;
    let t14228 = t10324 * t1662 * t2944;
    let t14232 = 0.13140859333333333334e-2_f64 * t3255 * t4572;
    (t14211, t14216, t14217, t14221, t14224, t14228, t14232)
}
