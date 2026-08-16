//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 759/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk759(t1086: f64, t2912: f64, t3323: f64, t3331: f64, t2921: f64, t846: f64, t1077: f64, t1079: f64, t12601: f64, t12604: f64, t12608: f64, t12624: f64, t142: f64, t15515: f64, t15522: f64, t15526: f64, t15537: f64, t15541: f64, t298: f64, t3306: f64, t3311: f64, t3312: f64, t3314: f64, t3324: f64, t3329: f64, t3332: f64, t3338: f64, t3342: f64, t3343: f64, t3346: f64, t3349: f64, t3350: f64) -> f64 {
    let t15545 = t1086 * t2912;
    let t15548 = t3323 * t3331;
    let t15552 = t2912 * t2921;
    let t15553 = t15552 * t846;
    let t15559 = 0.21687161765563048428e-1_f64 * t298 * t15515 * t1086 - 0.16265371324172286321e-1_f64 * t298 * t3338 * t3346 - 0.48159446095139119799e0_f64 * t298 * t15522 * t3350 + 0.68493333333333333332e-1_f64 * t298 * t15526 * t1079 - 0.51369999999999999999e-1_f64 * t298 * t3306 * t3324 + 0.10274e0_f64 * t298 * t142 * t3311 * t3314 + 0.32530742648344572643e-1_f64 * t298 * t15537 * t3343 - 0.16522997748472177549e1_f64 * t298 * t15541 * t3332 - 0.35089340384731224426e1_f64 * t3342 * t15545 + 0.96494049533612093922e2_f64 * t3329 * t15548 * t1077 + 0.51947267698127589897e2_f64 * t3349 * t15553 - 6.0_f64 * t3312 * t1079 * t3323 - t12601 + t12604 + t12608 - t12624;
    t15559
}
