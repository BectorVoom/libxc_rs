//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 954/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk954<F: Float>(t1175: F, t2191: F, t1364: F, t19351: F, t5704: F, t5953: F, t3593: F, t5658: F, t3619: F, t5932: F, t3564: F, t12848: F, t3540: F, t3592: F, t3545: F, t3521: F, t5934: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19352 = t2191 * t1175;
    let t19353 = t19352 * t1364;
    let t19354 = t19351 * t19353;
    let t19358 = t5704 * t1364;
    let t19359 = t5953 * t19358;
    let t19363 = t5658 * t3593;
    let t19364 = t5953 * t19363;
    let t19367 = t5932 * t3619;
    let t19368 = t3564 * t19367;
    let t19371 = t12848 * t2191;
    let t19372 = t19371 * t3540;
    let t19375 = t3592 * t2191;
    let t19376 = t19375 * t3545;
    let t19380 = 0.19711289e-2 * t3521 * t5934;
    (t19354, t19358, t19359, t19363, t19364, t19367, t19368, t19372, t19376, t19380)
}
