//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 632/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk632<F: Float>(t1882: F, t3983: F, t3839: F, t1140: F, t8232: F, t3848: F, t1170: F, t3953: F, t681: F, t89: F, t3856: F, t3974: F, t9735: F, t9701: F, t13746: F, t13753: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14212 = 2.0 / 9.0 * t1882 * t3983;
    let t14223 = 4.0 / 9.0 * t1882 * t3839;
    let t14224 = t8232 * t1140;
    let t14232 = 2.0 / 27.0 * t1882 * t3848;
    let t14233 = t8232 * t1170;
    let t14240 = 2.0 / 9.0 * t89 * t681 * t3953;
    let t14281 = 2.0 / 27.0 * t1882 * t3856;
    let t14283 = 2.0 / 9.0 * t1882 * t3974;
    let t14317 = 4.0 / 81.0 * t9735;
    let t14318 = 4.0 / 27.0 * t9701;
    let t14327 = 2.0 / 9.0 * t13746;
    let t14329 = t13753 / 9.0;
    (t14212, t14223, t14224, t14232, t14233, t14240, t14281, t14283, t14317, t14318, t14327, t14329)
}
