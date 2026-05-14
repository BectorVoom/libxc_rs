//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1021/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1021<F: Float>(t2388: F, t2379: F, t2385: F, t60: F, t81: F, t9260: F, t684: F, t9261: F, t20: F, t4879: F, t12230: F, t1360: F, t3951: F, t3960: F, t3716: F, t12229: F, t486: F, t506: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36957 = t2388 * t2388;
    let t36958 = 1.0 / t36957;
    let t36962 = t2379 * t2385;
    let t37000 = t60 / t9260 / t81;
    let t37013 = t684 * t9261;
    let t37041 = t4879 * t20;
    let t37602 = t1360 * t12230;
    let t37622 = t3951 * t3960;
    let t38629 = t3716 * t3716;
    let t38630 = 1.0 / t38629;
    let t39052 = t486 / t12229 / t506;
    (t36958, t36962, t37000, t37013, t37041, t37602, t37622, t38630, t39052)
}
