//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 632/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk632<F: Float>(t10568: F, t5101: F, t707: F, t1797: F, t180: F, t479: F, t574: F, t682: F, t695: F, t10459: F, t10463: F, t708: F, t4663: F, t213: F, t568: F, t5100: F, t680: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11371 = 0.12841111111111111111e-1 * t10568;
    let t11393 = t707 * t5101;
    let t11400 = t180 * t479 * t1797;
    let t11401 = t574 * t682;
    let t11402 = t11401 * t695;
    let t11417 = t10459 * t707;
    let t11418 = t708 * t10463;
    let t11443 = t4663 * t708;
    let t11458 = t213 * t568;
    let t11460 = 0.14055920378328537299e-1 * t11458 * t682;
    let t11480 = 1.0 / t5100 / t680;
    (t11371, t11393, t11400, t11402, t11417, t11418, t11443, t11458, t11460, t11480)
}
