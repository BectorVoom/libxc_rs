//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 947/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk947<F: Float>(t1433: F, t19221: F, t457: F, t1417: F, t5959: F, t3598: F, t416: F, t3559: F, t5927: F, t2226: F, t3517: F, t3521: F, t5945: F, t442: F, t5703: F, t1056: F) -> (F, F, F, F, F, F, F, F) {
    let t19222 = t1433 * t19221;
    let t19223 = t457 * t19222;
    let t19227 = 0.13140859333333333333e-2 * t1417 * t5959;
    let t19228 = t416 * t3598;
    let t19229 = t5927 * t3559;
    let t19230 = t19228 * t19229;
    let t19235 = t3517 * t2226;
    let t19237 = t3521 * t5945;
    let t19239 = t5703 * t442;
    let t19240 = t19239 * t1056;
    (t19222, t19223, t19227, t19229, t19230, t19235, t19237, t19240)
}
