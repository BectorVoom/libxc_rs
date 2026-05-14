//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 823/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk823<F: Float>(t4265: F, t5261: F, t1922: F, t979: F, t5256: F, t5265: F, t140: F, t446: F, t728: F, t299: F, t5268: F, t1925: F, t430: F, t1909: F, t574: F, t5172: F, t695: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11851 = t4265 * t5261;
    let t11853 = t979 * t1922;
    let t11855 = t4265 * t5256;
    let t11857 = t4265 * t5265;
    let t11885 = 0.11791604938271604938e-1 * t140 * t446 * t728;
    let t11891 = t140 * t299 * t5268;
    let t11894 = t140 * t430 * t1925;
    let t11900 = t1909 * t574;
    let t11916 = t5172 * t695;
    (t11851, t11853, t11855, t11857, t11885, t11891, t11894, t11900, t11916)
}
