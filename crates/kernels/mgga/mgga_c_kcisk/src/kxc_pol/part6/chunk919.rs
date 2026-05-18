//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 919/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk919<F: Float>(t29332: F, t5192: F, t5182: F, t28663: F, t6675: F, t6674: F, t16676: F, t8481: F, t6974: F, t8940: F, t1869: F, t1693: F, t24073: F, t28961: F, t28966: F, t28970: F, t28973: F, t29323: F, t29326: F, t29330: F) -> (F, F, F, F, F) {
    let t29333 = t5192 * t29332;
    let t29334 = t5182 * t29333;
    let t29336 = t6675 * t28663;
    let t29337 = t5192 * t29336;
    let t29338 = t6674 * t29337;
    let t29340 = t16676 * t8481;
    let t29342 = t6974 * t8940;
    let t29343 = t1869 * t29342;
    let t29346 = -F::new(0.11349419753086419753e0) * t28961 - F::new(0.1492375e-1) * t28966 - F::new(0.39796666666666666665e-1) * t28970 + F::new(0.49745833333333333332e-2) * t28973 - F::new(0.193e0) * t1693 * t29323 + F::new(0.66327777777777777776e-2) * t29326 + F::new(0.73697530864197530862e-3) * t29330 - F::new(0.99491666666666666664e-2) * t29334 + F::new(0.82909722222222222219e-2) * t29338 - F::new(0.99491666666666666664e-2) * t29340 - F::new(0.74618749999999999998e-2) * t29343 - F::new(0.2653111111111111111e-1) * t24073;
    (t29334, t29338, t29340, t29343, t29346)
}
