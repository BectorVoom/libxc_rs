//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1012/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1012(t4449: f64, t4474: f64, t15716: f64, t15793: f64, t15797: f64, t1594: f64, t1624: f64, t1631: f64, t20087: f64, t2021: f64, t3076: f64, t372: f64, t374: f64, t4467: f64, t4491: f64, t58911: f64, t73718: f64, t73912: f64, t7914: f64, t8042: f64, t85414: f64, t85618: f64, t85630: f64, t930: f64, t938: f64) -> f64 {
    let t85649 = t4449 * t4474;
    let t85679 = 0.46509801892875584e-1_f64 * t1624 * t374 * t930 * t20087 + 0.23238868087529279928e-2_f64 * t8042 * t1594 * t85649 + 0.77462893625097599764e-3_f64 * t372 * t1594 * t85618 + 0.1116235245429014016e-1_f64 * t1624 * t7914 * t85630 - 0.139529405678626752e0_f64 * t8042 * t374 * t4467 * t4474 + 0.38704743803858356237e-5_f64 * t372 * t2021 * t85414 + 0.81118562704294997116e-3_f64 * t15797 * t58911 - 36.0_f64 * t3076 * t15716 * t4491 + 8.0_f64 * t3076 * t73718 * t938 - 0.81118562704294997116e-3_f64 * t15793 * t73912 + 0.279058811357253504e-1_f64 * t8042 * t1631 * t85649;
    t85679
}
