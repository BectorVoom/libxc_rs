//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 425/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk425<F: Float>(t1078: F, t3323: F, t1070: F, t242: F, t250: F, t3313: F, t142: F, t841: F, t2900: F, t47: F, t2901: F, t848: F, t2912: F, t2918: F, t2921: F, t1072: F, t1079: F, t1085: F, t1086: F, t172: F, t251: F, t2849: F, t2853: F, t2860: F, t2882: F, t2890: F, t298: F, t3306: F, t3312: F, t3314: F, t5: F, t56: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3324 = t3323 * t1078;
    let t3327 = t1070 * t1070;
    let t3328 = 1.0 / t3327;
    let t3329 = t242 * t3328;
    let t3330 = t250 * t250;
    let t3331 = 1.0 / t3330;
    let t3332 = t3313 * t3331;
    let t3338 = t142 * t841;
    let t3342 = t47 * t2900;
    let t3343 = t2901 * t848;
    let t3346 = t2912 * t848;
    let t3349 = t47 * t2918;
    let t3350 = t2901 * t2921;
    let t3353 = -0.70981924444444444442e-3 * t5 * t172 * t251 - 0.34246666666666666666e-1 * t298 * t3306 * t1079 - 2.0 * t3312 * t3314 + 1.0 * t1072 * t3324 + 0.32164683177870697974e2 * t3329 * t3332 + t2849 + t2853 + t2860 - t2882 - t2890 - 0.24415406715670879921e-3 * t5 * t172 * t56 - 0.10843580882781524214e-1 * t298 * t3338 * t1086 - 0.11696446794910408142e1 * t3342 * t3343 + 0.58482233974552040708e0 * t1085 * t3346 + 0.17315755899375863299e2 * t3349 * t3350;
    (t3324, t3327, t3328, t3329, t3330, t3331, t3332, t3338, t3342, t3343, t3346, t3349, t3350, t3353)
}
