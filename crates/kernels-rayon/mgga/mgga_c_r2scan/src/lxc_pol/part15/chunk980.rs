//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 980/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk980(t10669: f64, t11506: f64, t3574: f64, t481: f64, t3263: f64, t10610: f64, t1100: f64, t2881: f64, t797: f64, t495: f64, t3579: f64, t3582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11507 = t11506 * t10669;
    let t11508 = 3.0_f64 / 4.0_f64 * t11507;
    let t11509 = t3574 * t481;
    let t11510 = t3263 * t11509;
    let t11511 = t10610 * t11510;
    let t11512 = 3.0_f64 / 2.0_f64 * t11511;
    let t11513 = t1100 * t2881;
    let t11514 = t3263 * t797;
    let t11515 = t495 * t11514;
    let t11516 = t3579 * t11515;
    let t11517 = t11516 / 4.0_f64;
    let t11518 = t3582 * t481;
    (t11507, t11508, t11509, t11510, t11511, t11512, t11513, t11514, t11515, t11516, t11517, t11518)
}
