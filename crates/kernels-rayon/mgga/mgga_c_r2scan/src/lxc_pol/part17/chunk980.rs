//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 980/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk980(t12033: f64, t3269: f64, t11325: f64, t3275: f64, t3582: f64, t1044: f64, t3560: f64, t11345: f64, t3579: f64, t11625: f64, t3465: f64, t11475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12034 = t3269 * t12033;
    let t12035 = t12034 / 4.0_f64;
    let t12037 = t3275 * t11325 * t3582;
    let t12038 = 5.0_f64 / 16.0_f64 * t12037;
    let t12039 = t3560 * t1044;
    let t12040 = t3579 * t11345;
    let t12041 = t12040 / 4.0_f64;
    let t12042 = t3465 * t11625;
    let t12043 = t3275 * t12042;
    let t12044 = t12043 / 2.0_f64;
    let t12045 = t3465 * t11475;
    (t12034, t12035, t12037, t12038, t12039, t12040, t12041, t12042, t12043, t12044, t12045)
}
