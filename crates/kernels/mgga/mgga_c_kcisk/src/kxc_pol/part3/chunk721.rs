//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 721/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk721<F: Float>(t11208: F, t1896: F, t1901: F, t4971: F, t654: F, t11154: F, t1800: F, t1869: F, t4597: F, t642: F, t1757: F, t3290: F) -> (F, F, F, F, F) {
    let t11209 = t11208 * t1896;
    let t11211 = t11208 * t1901;
    let t11213 = t654 * t4971;
    let t11214 = t11213 * t11154;
    let t11215 = t1800 * t11214;
    let t11216 = t1869 * t11215;
    let t11218 = t642 * t4597;
    let t11219 = t3290 * t1757;
    (t11209, t11211, t11216, t11218, t11219)
}
