//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1398/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1398(t1599: f64, t18163: f64, t12844: f64, t6155: f64, t4439: f64, t3970: f64, t617: f64, t5441: f64, t12140: f64, t5427: f64, t16069: f64, t6151: f64) -> (f64, f64, f64, f64, f64) {
    let t18164 = t1599 * t18163;
    let t18168 = t12844 * t6155;
    let t18170 = t4439 * t18168 / 864.0_f64;
    let t18171 = t3970 * t617;
    let t18172 = t18171 * t5441;
    let t18174 = t4439 * t18172 / 432.0_f64;
    let t18175 = t12140 * t617;
    let t18176 = t18175 * t5427;
    let t18178 = t4439 * t18176 / 648.0_f64;
    let t18179 = t6151 * t16069;
    (t18164, t18170, t18174, t18178, t18179)
}
