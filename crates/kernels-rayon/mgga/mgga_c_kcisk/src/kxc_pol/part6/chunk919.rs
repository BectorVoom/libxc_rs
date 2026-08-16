//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 919/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk919(t29332: f64, t5192: f64, t5182: f64, t28663: f64, t6675: f64, t6674: f64, t16676: f64, t8481: f64, t6974: f64, t8940: f64, t1869: f64, t1693: f64, t24073: f64, t28961: f64, t28966: f64, t28970: f64, t28973: f64, t29323: f64, t29326: f64, t29330: f64) -> (f64, f64, f64, f64, f64) {
    let t29333 = t5192 * t29332;
    let t29334 = t5182 * t29333;
    let t29336 = t6675 * t28663;
    let t29337 = t5192 * t29336;
    let t29338 = t6674 * t29337;
    let t29340 = t16676 * t8481;
    let t29342 = t6974 * t8940;
    let t29343 = t1869 * t29342;
    let t29346 = -0.11349419753086419753e0_f64 * t28961 - 0.1492375e-1_f64 * t28966 - 0.39796666666666666665e-1_f64 * t28970 + 0.49745833333333333332e-2_f64 * t28973 - 0.193e0_f64 * t1693 * t29323 + 0.66327777777777777776e-2_f64 * t29326 + 0.73697530864197530862e-3_f64 * t29330 - 0.99491666666666666664e-2_f64 * t29334 + 0.82909722222222222219e-2_f64 * t29338 - 0.99491666666666666664e-2_f64 * t29340 - 0.74618749999999999998e-2_f64 * t29343 - 0.2653111111111111111e-1_f64 * t24073;
    (t29334, t29338, t29340, t29343, t29346)
}
