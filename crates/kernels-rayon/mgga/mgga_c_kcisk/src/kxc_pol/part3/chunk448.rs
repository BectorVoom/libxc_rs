//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 448/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk448(t1425: f64, t3521: f64, t1417: f64, t1430: f64, t1435: f64, t313: f64, t442: f64) -> (f64, f64, f64, f64) {
    let t3522 = t3521 * t1425;
    let t3524 = t1417 * t1430;
    let t3526 = t1417 * t1435;
    let t3528 = t313 * t442;
    let t3529 = 1.0_f64 / t3528;
    (t3522, t3524, t3526, t3529)
}
