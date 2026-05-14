//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1034/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1034<F: Float>(t5641: F, t8232: F, t5650: F, t5661: F, t5712: F, t463: F, t5704: F, t38953: F, t5719: F, t1314: F, t3281: F, t5728: F, t92185: F, t93452: F, t5745: F, t1286: F, t1637: F, t5509: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93609 = t8232 * t5641;
    let t93612 = t8232 * t5650;
    let t93621 = t8232 * t5661;
    let t93630 = t8232 * t5712;
    let t93636 = t463 * t5704;
    let t93647 = t38953 * t5719;
    let t93676 = 28.0 / 81.0 * t3281 * t1314;
    let t93677 = t8232 * t5728;
    let t93728 = 14.0 / 27.0 * t92185;
    let t93776 = 28.0 / 27.0 * t93452;
    let t93828 = t8232 * t5745;
    let t93864 = t1286 * t1637 * t5509;
    (t93609, t93612, t93621, t93630, t93636, t93647, t93676, t93677, t93728, t93776, t93828, t93864)
}
