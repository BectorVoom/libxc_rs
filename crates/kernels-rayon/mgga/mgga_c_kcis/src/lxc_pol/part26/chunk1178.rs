//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1178/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1178(t684: f64, t9261: f64, t20: f64, t4879: f64, t12230: f64, t1360: f64, t3716: f64, t12229: f64, t486: f64, t506: f64, t12344: f64, t1502: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37013 = t684 * t9261;
    let t37041 = t4879 * t20;
    let t37602 = t1360 * t12230;
    let t38629 = t3716 * t3716;
    let t38630 = 1.0_f64 / t38629;
    let t39052 = t486 / t12229 / t506;
    let t39301 = t1502 * t12344;
    (t37013, t37041, t37602, t38630, t39052, t39301)
}
