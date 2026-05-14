//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1354/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1354<F: Float>(t2270: F, t32277: F, t6323: F, t3512: F, t8283: F, t113375: F, t9839: F, t27098: F, t32260: F, t109294: F, t8271: F, t27222: F, t33652: F, t1340: F, t27406: F, t27146: F) -> (F, F, F, F, F, F, F, F) {
    let t119771 = t2270 * t32277 * t6323;
    let t119773 = t3512 * t8283;
    let t119775 = t113375 * t9839;
    let t119777 = t32260 * t27098;
    let t119779 = t109294 * t8271;
    let t119781 = t33652 * t27222;
    let t119783 = t1340 * t27406;
    let t119785 = t1340 * t27146;
    (t119771, t119773, t119775, t119777, t119779, t119781, t119783, t119785)
}
