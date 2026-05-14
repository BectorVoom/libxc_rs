//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 548/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk548<F: Float>(t1340: F, t8093: F, t3759: F, t425: F, t7706: F, t2191: F, t5646: F, t7710: F, t3831: F, t7877: F, t1354: F, t7897: F, t1398: F, t7740: F, t1375: F, t7744: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8094 = t1340 * t8093;
    let t8095 = t3759 * t8094;
    let t8099 = t425 * t7706;
    let t8102 = t5646 * t2191;
    let t8105 = t425 * t7710;
    let t8108 = t3831 * t7877;
    let t8111 = t1354 * t7897;
    let t8123 = t1398 * t7740;
    let t8126 = t1375 * t7744;
    (t8094, t8095, t8099, t8102, t8105, t8108, t8111, t8123, t8126)
}
