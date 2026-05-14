//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 948/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk948<F: Float>(t22396: F, t7000: F, t2364: F, t4604: F, t6771: F, t2372: F, t6714: F, t4609: F, t6790: F, t3521: F, t8900: F, t8904: F, t4623: F, t695: F, t2063: F, t1648: F) -> (F, F, F, F, F, F, F, F) {
    let t22397 = t7000 * t22396;
    let t22401 = t4604 * t2364 * t6771;
    let t22405 = t4604 * t6714 * t2372;
    let t22409 = t4609 * t2364 * t6790;
    let t22412 = t3521 * t8900;
    let t22414 = t3521 * t8904;
    let t22416 = t4623 * t695;
    let t22417 = t2063 * t2372;
    let t22419 = t22416 * t22417 * t1648;
    (t22397, t22401, t22405, t22409, t22412, t22414, t22417, t22419)
}
