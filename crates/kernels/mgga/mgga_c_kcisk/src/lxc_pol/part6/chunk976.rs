//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 976/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk976<F: Float>(t19849: F, t8073: F, t1411: F, t5606: F, t7832: F, t10519: F, t10520: F, t30158: F, t8: F, t1450: F, t1340: F, t2075: F, t8247: F) -> (F, F, F, F, F) {
    let t30197 = t19849 * t8073;
    let t30198 = t1411 * t30197;
    let t30201 = t5606 * t7832;
    let t30202 = t1411 * t30201;
    let t30205 = t30158 * t8 + t10519 + t10520;
    let t30206 = t1450 * t30205;
    let t30207 = t1340 * t30206;
    let t30208 = t1411 * t30207;
    let t30212 = t8247 * t2075;
    (t30198, t30202, t30205, t30208, t30212)
}
