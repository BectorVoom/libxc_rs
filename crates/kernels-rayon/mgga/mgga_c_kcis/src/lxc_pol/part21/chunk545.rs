//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 545/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk545(t1254: f64, t3500: f64, t1251: f64, t1263: f64, t25: f64, t286: f64, t2887: f64, t2844: f64, t421: f64, t2630: f64, t283: f64, t414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3501 = t3500 * t1254;
    let t3502 = t1251 * t3501;
    let t3504 = t25 * t1263;
    let t3505 = t1251 * t3504;
    let t3507 = t286 * t2887;
    let t3508 = t421 * t2844;
    let t3509 = t3508 * t2630;
    let t3510 = t3507 * t3509;
    let t3513 = t414 * t283;
    (t3501, t3502, t3504, t3505, t3507, t3509, t3510, t3513)
}
