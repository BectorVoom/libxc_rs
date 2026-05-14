//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1356/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1356<F: Float>(t19861: F, t2279: F, t1506: F, t25351: F, t109241: F, t27123: F, t27201: F, t32260: F, t32287: F, t34869: F, t27067: F, t9491: F, t27227: F, t19951: F, t2275: F, t113421: F, t26774: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119795 = t19861 * t2279;
    let t119797 = t25351 * t1506;
    let t119799 = t109241 * t27123;
    let t119801 = t32260 * t27201;
    let t119803 = t32287 * t34869;
    let t119805 = t9491 * t27067;
    let t119807 = t9491 * t27227;
    let t119809 = t19951 * t2275;
    let t119811 = t113421 * t26774;
    (t119795, t119797, t119799, t119801, t119803, t119805, t119807, t119809, t119811)
}
