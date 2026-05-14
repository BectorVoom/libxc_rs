//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 246/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk246<F: Float>(t1297: F, t1303: F, t1311: F, t1315: F, t1323: F, t1326: F, t1327: F, t1330: F, t255: F, t261: F, t262: F, t331: F, t831: F) -> (F,) {
    let t1338 = 2.0 * t1297 * t255 - 1.0 * t1303 * t255 + 1.0 * t1311 * t255 + 0.2845018947250181111e-1 * t1315 * t331 - 0.20235332025531322028e-2 * t1323 * t1326 * t1327 * t1330 + 0.52158680699586653702e-1 * t261 * t262 * t831;
    (t1338,)
}
