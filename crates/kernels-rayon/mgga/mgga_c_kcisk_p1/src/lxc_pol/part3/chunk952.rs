//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 952/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk952(t1390: f64, t213: f64, t1056: f64, t3283: f64, t3859: f64, t1387: f64, t12830: f64, t12924: f64, t1349: f64, t1391: f64, t14083: f64, t14084: f64, t14085: f64, t14088: f64, t14091: f64, t14093: f64, t14096: f64) -> f64 {
    let t14100 = t213 * t1390;
    let t14101 = t14100 * t1056;
    let t14103 = t3859 * t3283;
    let t14107 = t1387 * t1056;
    let t14109 = -t14083 + t14084 - 0.62154466893555682512e-3_f64 * t14085 * t12830 + 0.71734315950379065738e-1_f64 * t14088 - 0.93231700340333523768e-3_f64 * t14091 + 0.71734315950379065738e-1_f64 * t14093 * t12830 - 0.35867157975189532869e-1_f64 * t14096 + 0.11955719325063177623e-1_f64 * t1349 * t12924 - 0.93231700340333523768e-3_f64 * t14101 + 0.31077233446777841256e-3_f64 * t14103 - 0.5179538907796306876e-4_f64 * t1391 * t12924 + 0.71734315950379065738e-1_f64 * t14107;
    t14109
}
