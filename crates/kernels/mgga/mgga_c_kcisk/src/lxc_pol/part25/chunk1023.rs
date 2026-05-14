//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1023/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1023<F: Float>(t2543: F, t574: F, t4265: F, t7391: F, t1909: F, t707: F, t16017: F, t7378: F, t16004: F, t7370: F, t1797: F, t1860: F, t6764: F, t11851: F, t11853: F, t11855: F, t11857: F, t16866: F, t16904: F, t16909: F, t16913: F, t16918: F, t16922: F, t4631: F, t5231: F, t6278: F, t7056: F, t7349: F, t7360: F) -> (F,) {
    let t18089 = t2543 * t574;
    let t18092 = t4265 * t7391;
    let t18100 = t1909 * t707;
    let t18113 = t7378 * t16017;
    let t18116 = t7370 * t16004;
    let t18119 = t1797 * t1860;
    let t18120 = t18119 * t6764;
    let t18123 = -0.1857375e-1 * t18089 * t4631 - 0.35374814814814814814e-1 * t18092 + 0.9286875e-2 * t7349 * t16909 + 0.46434375e-2 * t7349 * t16913 - 0.1857375e-1 * t5231 * t16918 + 0.24765e-1 * t18100 * t7056 - 0.9286875e-2 * t5231 * t16866 + 0.35374814814814814814e-1 * t11851 + 0.11791604938271604938e-1 * t11853 - 0.35374814814814814814e-1 * t11855 - 0.17687407407407407407e-1 * t11857 - 0.9286875e-2 * t5231 * t16922 + 0.123825e-1 * t7360 * t16904 - 0.15918666666666666667e0 * t6278 * t18113 + 0.26531111111111111111e0 * t6278 * t18116 + 0.10612444444444444444e0 * t6278 * t18120;
    (t18123,)
}
