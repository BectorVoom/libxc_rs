//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 794/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk794<F: Float>(t3587: F, t425: F, t1364: F, t3564: F, t3565: F, t3619: F, t12825: F, t458: F, t12829: F, t459: F, t12830: F, t1175: F, t1354: F, t3593: F, t3521: F, t3551: F) -> (F, F, F, F, F, F, F, F) {
    let t13211 = t425 * t3587;
    let t13212 = t13211 * t1364;
    let t13213 = t3564 * t13212;
    let t13216 = t3565 * t3619;
    let t13217 = t3564 * t13216;
    let t13220 = t12825 * t458;
    let t13221 = t459 * t12829;
    let t13223 = t13220 * t13221 * t12830;
    let t13227 = t1354 * t1175 * t3593;
    let t13228 = t3564 * t13227;
    let t13231 = t3521 * t3551;
    (t13212, t13213, t13216, t13217, t13223, t13227, t13228, t13231)
}
