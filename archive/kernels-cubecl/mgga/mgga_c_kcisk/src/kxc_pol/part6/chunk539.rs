//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 539/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk539<F: Float>(t707: F, t725: F, t2551: F, t4265: F, t4594: F, t702: F, t1797: F, t140: F, t2554: F, t299: F, t2505: F, t695: F) -> (F, F, F, F, F, F) {
    let t7360 = t725 * t707;
    let t7368 = t4265 * t2551;
    let t7370 = t4594 * t702;
    let t7378 = t1797 * t702;
    let t7387 = t140 * t299 * t2554;
    let t7389 = t2505 * t695;
    (t7360, t7368, t7370, t7378, t7387, t7389)
}
