//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 706/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk706<F: Float>(t10593: F, t7378: F, t140: F, t446: F, t728: F, t299: F, t5268: F, t1925: F, t430: F, t11638: F, t673: F, t1909: F, t574: F, t1860: F, t4597: F, t3290: F, t5248: F) -> (F, F, F, F, F, F, F) {
    let t11880 = t7378 * t10593;
    let t11885 = 0.11791604938271604938e-1 * t140 * t446 * t728;
    let t11891 = t140 * t299 * t5268;
    let t11894 = t140 * t430 * t1925;
    let t11896 = t673 * t11638;
    let t11900 = t1909 * t574;
    let t11905 = t1860 * t4597;
    let t11907 = t5248 * t11905 * t3290;
    (t11880, t11885, t11891, t11894, t11896, t11900, t11907)
}
