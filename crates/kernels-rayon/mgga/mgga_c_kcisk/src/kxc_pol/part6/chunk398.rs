//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 398/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk398(t776: f64, t2399: f64, t41: f64, t2442: f64, t525: f64, t642: f64, t773: f64, t79: f64, t781: f64) -> (f64, f64, f64) {
    let t777 = t776 < -0.66725e-1_f64;
    let t2620 = t2399 * t41;
    let t2628 = piecewise3(t777, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t2620 * t642 - 10.0_f64 / 27.0_f64 * t525 * t773 * t2442);
    let t2629 = t79 * t2628;
    let t2630 = t2629 * t781;
    (t2620, t2629, t2630)
}
