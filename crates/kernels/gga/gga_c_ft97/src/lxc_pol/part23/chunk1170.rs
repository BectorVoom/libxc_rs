//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1170/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1170<F: Float>(t2347: F, t7021: F, t2360: F, t1882: F, t28797: F, t11176: F, t1485: F, t28757: F, t28822: F, t6308: F, t681: F, t28512: F, t1486: F, t28497: F, t28525: F, t28817: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113279 = t7021 * t2347;
    let t113286 = t7021 * t2360;
    let t113295 = t1882 * t28797;
    let t113296 = 2.0 / 9.0 * t113295;
    let t113298 = t1485 * t11176 * t28757;
    let t113325 = t6308 * t681 * t28822;
    let t113326 = t113325 / 6.0;
    let t113329 = t6308 * t681 * t28512;
    let t113330 = t113329 / 6.0;
    let t113332 = t1486 * t681 * t28497;
    let t113333 = 2.0 / 3.0 * t113332;
    let t113339 = t1882 * t28525;
    let t113340 = 4.0 / 27.0 * t113339;
    let t113346 = t6308 * t681 * t28817;
    (t113279, t113286, t113295, t113296, t113298, t113325, t113326, t113329, t113330, t113332, t113333, t113339, t113340, t113346)
}
