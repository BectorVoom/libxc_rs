//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 395/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk395<F: Float>(t2532: F, t719: F, t717: F, t415: F, t1899: F, t2441: F) -> (F, F, F, F) {
    let t2533 = t2532 * t719;
    let t2534 = t717 * t2533;
    let t2535 = t415 * t2534;
    let t2537 = t1899 * t2441;
    (t2533, t2534, t2535, t2537)
}
