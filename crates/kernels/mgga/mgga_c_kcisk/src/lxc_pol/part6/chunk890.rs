//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 890/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk890<F: Float>(t1212: F, t30551: F, t3722: F, t19580: F, t7754: F, t2092: F, t7753: F, t1191: F, t3677: F, t2093: F, t7785: F, t3639: F, t25894: F, t1471: F, t30294: F, t12: F) -> (F, F, F, F, F, F, F) {
    let t30553 = t3722 * t30551 * t1212;
    let t30557 = 6.0 * t19580 * t7754;
    let t30558 = t7753 * t2092;
    let t30559 = t30558 * t1191;
    let t30561 = 6.0 * t3677 * t30559;
    let t30562 = t2093 * t7785;
    let t30564 = 6.0 * t3639 * t30562;
    let t30565 = t25894 * t2092;
    let t30567 = 0.48245472966453314466e2 * t3677 * t30565;
    let t30568 = t1471 * t30294;
    let t30569 = t12 * t30568;
    (t30553, t30557, t30558, t30561, t30564, t30567, t30569)
}
