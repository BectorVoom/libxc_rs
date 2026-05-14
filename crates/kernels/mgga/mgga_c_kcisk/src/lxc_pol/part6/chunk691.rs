//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 691/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk691<F: Float>(t2912: F, t2921: F, t846: F, t1077: F, t1079: F, t1086: F, t12601: F, t12604: F, t12608: F, t12624: F, t142: F, t15515: F, t15522: F, t15526: F, t15537: F, t15541: F, t15545: F, t15548: F, t298: F, t3306: F, t3311: F, t3312: F, t3314: F, t3323: F, t3324: F, t3329: F, t3332: F, t3338: F, t3342: F, t3343: F, t3346: F, t3349: F, t3350: F) -> (F,) {
    let t15552 = t2912 * t2921;
    let t15553 = t15552 * t846;
    let t15559 = 0.21687161765563048428e-1 * t298 * t15515 * t1086 - 0.16265371324172286321e-1 * t298 * t3338 * t3346 - 0.48159446095139119799e0 * t298 * t15522 * t3350 + 0.68493333333333333332e-1 * t298 * t15526 * t1079 - 0.51369999999999999999e-1 * t298 * t3306 * t3324 + 0.10274e0 * t298 * t142 * t3311 * t3314 + 0.32530742648344572643e-1 * t298 * t15537 * t3343 - 0.16522997748472177549e1 * t298 * t15541 * t3332 - 0.35089340384731224426e1 * t3342 * t15545 + 0.96494049533612093922e2 * t3329 * t15548 * t1077 + 0.51947267698127589897e2 * t3349 * t15553 - 6.0 * t3312 * t1079 * t3323 - t12601 + t12604 + t12608 - t12624;
    (t15559,)
}
