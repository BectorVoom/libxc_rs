//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1344/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1344<F: Float>(t1486: F, t31573: F, t681: F, t18514: F, t99352: F, t43468: F, t446: F, t18497: F, t25037: F, t10409: F, t3281: F, t193: F, t31551: F, t6308: F, t852: F, t856: F) -> (F, F, F, F, F, F) {
    let t126832 = t1486 * t681 * t31573;
    let t126833 = t99352 * t18514;
    let t126835 = t446 * t43468 * t126833;
    let t126837 = t25037 * t18497;
    let t126839 = t3281 * t10409 * t126837;
    let t126844 = t6308 * t193 * t852 * t31551 * t856;
    (t126832, t126833, t126835, t126837, t126839, t126844)
}
