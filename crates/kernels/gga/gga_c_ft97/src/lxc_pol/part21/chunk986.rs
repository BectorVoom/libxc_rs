//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 986/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk986<F: Float>(t1391: F, t4462: F, t569: F, t2205: F, t4454: F, t144: F, t30128: F, t1384: F, t4668: F, t2185: F, t605: F, t1017: F, t574: F, t6725: F, t1060: F, t6630: F) -> (F, F, F, F, F, F, F) {
    let t30508 = t569 * t1391 * t4462;
    let t30512 = t2205 * t1391 * t4454;
    let t30515 = t144 * t30128;
    let t30518 = t1384 * t4668;
    let t30520 = t2185 * t605 * t30518;
    let t30524 = t574 * t6725 * t1017;
    let t30528 = t2185 * t1060 * t6630;
    (t30508, t30512, t30515, t30518, t30520, t30524, t30528)
}
