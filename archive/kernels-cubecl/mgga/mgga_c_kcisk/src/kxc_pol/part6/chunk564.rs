//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 564/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk564<F: Float>(t2075: F, t2191: F, t3544: F, t1422: F, t3549: F, t7706: F, t1423: F, t7710: F, t3558: F, t7757: F, t457: F, t5932: F) -> (F, F, F, F, F, F) {
    let t7853 = t2075 * t2191;
    let t7854 = t3544 * t7853;
    let t7858 = t1422 * t3549 * t7706;
    let t7862 = t1422 * t1423 * t7710;
    let t7865 = t3558 * t7757;
    let t7866 = t457 * t7865;
    let t7869 = t5932 * t2191;
    (t7854, t7858, t7862, t7865, t7866, t7869)
}
