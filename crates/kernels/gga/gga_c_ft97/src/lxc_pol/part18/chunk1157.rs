//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1157/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1157<F: Float>(t100378: F, t1564: F, t446: F, t100349: F, t38262: F, t1647: F, t6469: F, t7824: F, t1317: F, t3000: F, t469: F, t5691: F, t26017: F, t376: F, t5665: F, t1307: F, t38463: F) -> (F, F, F, F, F, F, F, F) {
    let t100395 = t446 * t1564 * t100378;
    let t100398 = t446 * t38262 * t100349;
    let t100400 = t6469 * t1647;
    let t100402 = t446 * t7824 * t100400;
    let t100406 = t1317 * t3000 * t469 * t5691;
    let t100409 = t5665 * t376 * t26017;
    let t100410 = t100409 / 18.0;
    let t100411 = t38463 * t1307;
    (t100395, t100398, t100400, t100402, t100406, t100409, t100410, t100411)
}
