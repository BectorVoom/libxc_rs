//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 888/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk888<F: Float>(t13300: F, t3796: F, t3482: F, t1337: F, t1404: F, t1336: F, t140: F, t3800: F, t3488: F, t1299: F, t3483: F, t3487: F) -> (F, F, F, F) {
    let t13301 = t3796 * t13300;
    let t13302 = t3482 * t13301;
    let t13304 = t1337 * t1404;
    let t13306 = t140 * t1336 * t13304;
    let t13307 = t13306 * t3800;
    let t13309 = t13306 * t3488;
    let t13311 = t3483 * t1299;
    let t13312 = t13311 * t3487;
    (t13302, t13307, t13309, t13312)
}
