//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 943/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk943(t13959: f64, t3488: f64, t1333: f64, t3909: f64, t213: f64, t300: f64, t425: f64, t1350: f64, t1387: f64, t1365: f64, t3812: f64, t3827: f64, t443: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13960 = t13959 * t3488;
    let t13962 = t1333 * t3909;
    let t13964 = t213 * t300;
    let t13966 = 0.14055920378328537299e-1_f64 * t13964 * t425;
    let t13967 = t1387 * t1350;
    let t13969 = t3812 * t1365;
    let t13971 = t443 * t3827;
    (t13960, t13962, t13964, t13966, t13967, t13969, t13971)
}
