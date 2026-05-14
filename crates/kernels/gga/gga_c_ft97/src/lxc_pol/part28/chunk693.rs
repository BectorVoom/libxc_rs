//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 693/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk693<F: Float>(t32355: F, t5508: F, t28: F, t1308: F, t5748: F, t376: F, t7167: F, t1286: F, t32338: F, t22917: F, t5507: F, t1332: F, t5743: F, t1852: F, t492: F, t7281: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32391 = t32355 * t5508;
    let t32392 = t28 * t32391;
    let t32395 = t1308 * t5748;
    let t32396 = t28 * t32395;
    let t32399 = t376 * t7167;
    let t32401 = t1286 * t32399 / 9.0;
    let t32402 = t32338 * t5508;
    let t32403 = t28 * t32402;
    let t32405 = t5507 * t22917;
    let t32406 = t28 * t32405;
    let t32411 = t1332 * t5743;
    let t32412 = t1852 * t32411;
    let t32414 = t7281 * t492;
    (t32391, t32392, t32395, t32396, t32399, t32401, t32402, t32403, t32405, t32406, t32411, t32412, t32414)
}
