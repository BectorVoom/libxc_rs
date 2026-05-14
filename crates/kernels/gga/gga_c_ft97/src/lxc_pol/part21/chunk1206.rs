//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1206/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1206<F: Float>(t1882: F, t29845: F, t29836: F, t103350: F, t11490: F, t11837: F, t1332: F, t15885: F, t16093: F, t16286: F, t16291: F, t1871: F, t1901: F, t23249: F, t23294: F, t26145: F, t26372: F, t26373: F, t29802: F, t29943: F, t3193: F, t3238: F, t3291: F, t38711: F, t4454: F, t446: F, t452: F, t488: F, t6469: F, t6538: F, t8506: F, t91895: F, t91897: F) -> (F,) {
    let t117707 = t1882 * t29845;
    let t117709 = t1882 * t29836;
    let t117746 = -4.0 / 3.0 * t1901 * t11490 * t23249 * t16093 + t1901 * t8506 * t29802 / 9.0 + t117707 / 27.0 + 2.0 / 3.0 * t117709 + 4.0 / 3.0 * t446 * t1871 * t3291 * t6469 + 4.0 / 27.0 * t103350 - 4.0 / 81.0 * t91895 + 4.0 / 27.0 * t91897 + 2.0 / 27.0 * t1901 * t3193 * t23294 * t4454 - 2.0 / 3.0 * t1901 * t11490 * t23249 * t16286 - 2.0 * t1901 * t26372 * t26373 * t16291 - 2.0 / 9.0 * t1901 * t38711 * t29943 + 2.0 / 3.0 * t446 * t452 * t11837 * t6538 + 2.0 / 3.0 * t446 * t452 * t3238 * t26145 + t446 * t452 * t488 * t1332 * t15885 / 3.0;
    (t117746,)
}
