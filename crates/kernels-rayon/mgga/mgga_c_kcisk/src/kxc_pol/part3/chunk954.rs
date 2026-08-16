//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 954/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk954(t1391: f64, t3820: f64, t3824: f64, t443: f64, t1346: f64, t3832: f64, t12830: f64, t425: f64, t1354: f64, t3278: f64, t1364: f64, t1350: f64, t3283: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14116 = t1391 * t3820;
    let t14118 = t443 * t3824;
    let t14120 = t1346 * t3832;
    let t14122 = t425 * t12830;
    let t14125 = t1354 * t3278;
    let t14126 = t14125 * t1364;
    let t14129 = t1350 * t3283;
    (t14116, t14118, t14120, t14122, t14126, t14129)
}
