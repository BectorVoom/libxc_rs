//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 461/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk461(t1337: f64, t140: f64, t3737: f64, t1336: f64, t3529: f64, t1284: f64, t394: f64, t1412: f64, t466: f64) -> (f64, f64, f64, f64) {
    let t3748 = t140 * t3737 * t1337;
    let t3759 = t140 * t1336 * t3529;
    let t3776 = t394 * t1284;
    let t3783 = 1.0_f64 / t1412 / t466;
    (t3748, t3759, t3776, t3783)
}
