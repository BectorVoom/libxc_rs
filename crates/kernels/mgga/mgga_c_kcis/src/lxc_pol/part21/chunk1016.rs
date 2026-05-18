//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1016/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1016<F: Float>(t4731: F, t962: F, t1684: F, t3031: F, t14026: F, t971: F, t1823: F, t3549: F, t10869: F, t10888: F, t1212: F, t1225: F, t1226: F, t13854: F, t15369: F, t15423: F, t15442: F, t1831: F, t1835: F, t3545: F, t3552: F, t3578: F, t3582: F, t3589: F, t3593: F, t405: F, t5234: F, t5242: F, t5250: F) -> F {
    let t15445 = t4731 * t962;
    let t15450 = t1684 * t3031;
    let t15457 = t14026 * t971;
    let t15460 = t1823 * t3549;
    let t15463 = F::new(0.32164683177870697974e2) * t15369 * t3578 + F::new(1.0) * t10888 * t1831 + F::new(2.0) * t3545 * t5234 + F::new(1.0) * t1212 * t15423 - F::new(0.3109e-1) * t15442 * t405 + F::new(0.11696446794910408142e1) * t15445 * t1226 + F::new(0.58482233974552040708e0) * t5242 * t3589 + F::new(0.17315755899375863299e2) * t15450 * t3593 + F::new(0.58482233974552040708e0) * t10869 * t1835 + F::new(0.11696446794910408142e1) * t3582 * t5250 + F::new(0.58482233974552040708e0) * t1225 * t15457 - F::new(2.0) * t15460 * t3552 + t13854;
    t15463
}
