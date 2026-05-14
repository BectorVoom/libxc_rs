//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 963/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk963<F: Float>(t28860: F, t296: F, t4167: F, t6353: F, t840: F, t1508: F, t2862: F, t4162: F, t1882: F, t7055: F, t28845: F, t28848: F, t4246: F, t6365: F, t28850: F, t1501: F, t4129: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29342 = t296 * t28860;
    let t29346 = t840 * t6353 * t4167;
    let t29350 = t2862 * t1508 * t4162;
    let t29354 = t1882 * t7055;
    let t29356 = t296 * t28845;
    let t29359 = t296 * t28848;
    let t29363 = t840 * t4246 * t6365;
    let t29366 = t296 * t28850;
    let t29369 = t1501 * t4129;
    (t29342, t29346, t29350, t29354, t29356, t29359, t29363, t29366, t29369)
}
