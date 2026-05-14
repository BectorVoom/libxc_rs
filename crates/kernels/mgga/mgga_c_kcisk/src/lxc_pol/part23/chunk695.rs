//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 695/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk695<F: Float>(t1313: F, t220: F, t6187: F, t2169: F, t25: F, t1309: F, t2168: F, t442: F, t1056: F, t3937: F, t1310: F, t398: F) -> (F, F, F, F, F, F, F, F) {
    let t6188 = t1313 * t220;
    let t6189 = t6187 * t6188;
    let t6196 = t25 * t2169;
    let t6197 = t1309 * t6196;
    let t6199 = t2168 * t442;
    let t6200 = t6199 * t1056;
    let t6201 = t3937 * t6200;
    let t6204 = t1310 * t398;
    (t6188, t6189, t6196, t6197, t6199, t6200, t6201, t6204)
}
