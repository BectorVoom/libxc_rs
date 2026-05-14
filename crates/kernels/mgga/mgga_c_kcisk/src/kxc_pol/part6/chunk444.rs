//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 444/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk444<F: Float>(t1337: F, t140: F, t3737: F, t1336: F, t3529: F, t1284: F, t394: F, t1412: F, t466: F) -> (F, F, F, F) {
    let t3748 = t140 * t3737 * t1337;
    let t3759 = t140 * t1336 * t3529;
    let t3776 = t394 * t1284;
    let t3783 = 1.0 / t1412 / t466;
    (t3748, t3759, t3776, t3783)
}
