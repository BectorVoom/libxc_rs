//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1233/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1233<F: Float>(t28512: F, t6308: F, t681: F, t1486: F, t28497: F, t2682: F, t7021: F, t10570: F, t193: F, t1882: F, t28525: F, t13863: F, t25037: F, t2665: F, t446: F, t28817: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t113329 = t6308 * t681 * t28512;
    let t113330 = t113329 / 6.0;
    let t113332 = t1486 * t681 * t28497;
    let t113333 = 2.0 / 3.0 * t113332;
    let t113334 = t7021 * t2682;
    let t113337 = t1486 * t193 * t10570 * t113334;
    let t113339 = t1882 * t28525;
    let t113340 = 4.0 / 27.0 * t113339;
    let t113341 = t25037 * t13863;
    let t113343 = t446 * t2665 * t113341;
    let t113346 = t6308 * t681 * t28817;
    (t113329, t113330, t113332, t113333, t113334, t113337, t113339, t113340, t113341, t113343, t113346)
}
