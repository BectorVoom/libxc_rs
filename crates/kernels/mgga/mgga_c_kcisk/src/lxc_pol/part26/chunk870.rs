//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 870/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk870<F: Float>(t1417: F, t5938: F, t12848: F, t2083: F, t2222: F, t3517: F, t2188: F, t3598: F, t1173: F, t5700: F, t19102: F, t5959: F, t416: F, t2226: F, t3521: F, t5945: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19150 = 0.19711289e-2 * t1417 * t5938;
    let t19155 = t12848 * t2083;
    let t19163 = t3517 * t2222;
    let t19182 = t3598 * t2188;
    let t19185 = t1173 * t5700;
    let t19199 = 0.18344444444444444444e-2 * t19102;
    let t19227 = 0.13140859333333333333e-2 * t1417 * t5959;
    let t19228 = t416 * t3598;
    let t19235 = t3517 * t2226;
    let t19237 = t3521 * t5945;
    (t19150, t19155, t19163, t19182, t19185, t19199, t19227, t19228, t19235, t19237)
}
