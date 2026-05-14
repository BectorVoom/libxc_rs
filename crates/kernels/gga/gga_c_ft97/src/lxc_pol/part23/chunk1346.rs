//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1346/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1346<F: Float>(t126397: F, t2665: F, t446: F, t18514: F, t99322: F, t10409: F, t126401: F, t3281: F, t25027: F, t31578: F, t681: F, t31583: F, t6308: F, t113298: F, t113326: F, t113330: F, t113333: F, t113340: F, t113347: F, t126854: F) -> (F, F, F, F, F, F, F) {
    let t126857 = t446 * t2665 * t126397;
    let t126859 = t99322 * t18514;
    let t126861 = t446 * t10409 * t126859;
    let t126864 = t3281 * t2665 * t126401;
    let t126867 = t25027 * t681 * t31578;
    let t126868 = t126867 / 8.0;
    let t126870 = t6308 * t681 * t31583;
    let t126871 = t126870 / 12.0;
    let t126872 = 2.0 / 9.0 * t113298 + 4.0 / 3.0 * t126854 + 2.0 * t126857 - 4.0 / 3.0 * t126861 + 8.0 / 3.0 * t126864 - t113326 - t113330 - t113333 - t113340 - t113347 + t126868 - t126871;
    (t126857, t126859, t126861, t126864, t126867, t126870, t126872)
}
