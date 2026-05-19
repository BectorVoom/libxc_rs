//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 683/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk683<F: Float>(t11401: F, t695: F, t10459: F, t707: F, t10463: F, t708: F, t4663: F, t213: F, t568: F, t682: F, t5100: F, t680: F) -> (F, F, F, F, F, F, F) {
    let t11402 = t11401 * t695;
    let t11417 = t10459 * t707;
    let t11418 = t708 * t10463;
    let t11443 = t4663 * t708;
    let t11458 = t213 * t568;
    let t11460 = F::cast_from(0.14055920378328537299e-1_f64) * t11458 * t682;
    let t11480 = F::new(1.0) / t5100 / t680;
    (t11402, t11417, t11418, t11443, t11458, t11460, t11480)
}
