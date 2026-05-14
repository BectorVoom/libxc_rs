//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 782/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk782<F: Float>(t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12948: F, t12954: F, t12959: F, t12975: F, t12985: F, t12989: F, t1173: F, t1180: F, t311: F, t313: F, t3841: F) -> (F, F, F, F) {
    let t12992 = -t12975 - 4.0 / 9.0 * t12929 + 2.0 / 9.0 * t12933 - 2.0 / 3.0 * t12948 + t12931 / 3.0 - 10.0 / 27.0 * t12922 + 4.0 / 3.0 * t12954 - 2.0 / 3.0 * t12985 - 2.0 * t12959 + 2.0 * t12989 - t12927 / 3.0;
    let t12993 = t1173 * t12992;
    let t12995 = t1180 * t12992;
    let t12998 = t311 * t3841 * t313;
    (t12992, t12993, t12995, t12998)
}
