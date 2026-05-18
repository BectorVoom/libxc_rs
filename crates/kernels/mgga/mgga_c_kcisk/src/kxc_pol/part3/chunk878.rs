//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 878/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk878<F: Float>(t1163: F, t3619: F, t3544: F, t1417: F, t3589: F, t3595: F, t3598: F, t459: F, t12970: F, t457: F, t3621: F, t3521: F, t3567: F) -> (F, F, F, F, F, F, F) {
    let t13175 = t1163 * t3619;
    let t13176 = t3544 * t13175;
    let t13179 = t1417 * t3589;
    let t13183 = t1417 * t3595;
    let t13185 = t3598 * t459;
    let t13186 = t13185 * t12970;
    let t13187 = t457 * t13186;
    let t13190 = t1417 * t3621;
    let t13192 = t3521 * t3567;
    (t13176, t13179, t13183, t13186, t13187, t13190, t13192)
}
