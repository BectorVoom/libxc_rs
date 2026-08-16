//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 504/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk504(t338: f64, t397: f64, t3979: f64, t403: f64, t396: f64, t1323: f64, t25: f64, t1309: f64, t3729: f64, t1320: f64, t1310: f64, t1293: f64, t1300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t400 = 0.0_f64 < t338;
    let t3981 = t397 * t3979 * t403;
    let t3983 = 0.11993859144118211475e-1_f64 * t396 * t3981;
    let t3984 = t25 * t1323;
    let t3985 = t1309 * t3984;
    let t3988 = piecewise3(t400, t3729, -t3729);
    let t3989 = t1320 * t3988;
    let t3990 = t1310 * t3989;
    let t3993 = t1293 * t1300;
    (t3981, t3983, t3984, t3985, t3988, t3989, t3990, t3993)
}
