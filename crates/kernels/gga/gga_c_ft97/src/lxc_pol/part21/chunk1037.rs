//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1037/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1037<F: Float>(t32772: F, t5585: F, t5818: F, t2001: F, t23773: F, t22511: F, t23823: F, t23704: F, t23772: F, t444: F, t3392: F, t23831: F, t23700: F, t1355: F, t93191: F, t92529: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t94375 = t32772 * t5585;
    let t94376 = t5818 * t94375;
    let t94387 = t2001 * t23773;
    let t94400 = t23823 * t22511;
    let t94401 = t2001 * t94400;
    let t94429 = t5818 * t94400;
    let t94434 = t2001 * t23704;
    let t94507 = t23772 * t444;
    let t94508 = t3392 * t94507;
    let t94514 = t23831 * t94400;
    let t94524 = t3392 * t94400;
    let t94530 = t23831 * t23700;
    let t94535 = t2001 * t23700;
    let t94578 = t1355 * t93191;
    let t94600 = 0.18521666970164609055e-1 * t1355 * t92529;
    (t94375, t94376, t94387, t94401, t94429, t94434, t94507, t94508, t94514, t94524, t94530, t94535, t94578, t94600)
}
