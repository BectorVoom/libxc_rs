//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1068/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1068<F: Float>(t1823: F, t3574: F, t13908: F, t13720: F, t13726: F, t13729: F, t13735: F, t13738: F, t9700: F, t9702: F, t9708: F, t9710: F, t9712: F, t13712: F, t10923: F, t10924: F, t13710: F, t13714: F, t13723: F, t13732: F, t13767: F, t13942: F, t13945: F, t13949: F) -> (F, F, F) {
    let t15369 = t1823 * t3574;
    let t15397 = 0.27785333333333333334e0 * t13908;
    let t15398 = -0.34431666666666666666e0 * t9700 - 0.13892666666666666667e0 * t9702 - 0.23154444444444444444e0 * t9708 + 0.69463333333333333333e-1 * t9710 + 0.23154444444444444444e-1 * t9712 - 0.34431666666666666667e0 * t13729 - 0.57386111111111111112e0 * t13720 - 0.13772666666666666667e1 * t13726 + 0.103295e1 * t13738 + 0.41318e1 * t13735 - t15397;
    let t15411 = 0.22954444444444444444e0 * t13712;
    let t15420 = t15411 - 0.68863333333333333333e0 * t13714 + 0.20659e1 * t13723 - 0.309885e1 * t13732 - t10923 - t10924 + 0.6311625e0 * t13942 + 0.3529725e1 * t13767 - 0.11577222222222222222e0 * t13945 - 0.22954444444444444444e0 * t13710 + 0.90302333333333333334e0 * t13949;
    (t15369, t15398, t15420)
}
