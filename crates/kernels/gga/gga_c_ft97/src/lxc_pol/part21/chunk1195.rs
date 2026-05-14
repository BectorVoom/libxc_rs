//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1195/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1195<F: Float>(t1882: F, t29833: F, t30013: F, t8392: F, t29979: F, t29975: F, t102487: F, t103: F, t103423: F, t11472: F, t11556: F, t11593: F, t117226: F, t16011: F, t16093: F, t16150: F, t16155: F, t16160: F, t16169: F, t1901: F, t26435: F, t26440: F, t28: F, t39167: F, t446: F, t452: F, t47399: F, t47799: F, t5710: F, t82: F, t89: F, t91625: F) -> (F,) {
    let t117256 = t1882 * t29833;
    let t117270 = t8392 * t30013;
    let t117272 = t1882 * t29979;
    let t117274 = t1882 * t29975;
    let t117276 = t89 * t28 * t82 * t117226 * t103 / 3.0 + 2.0 / 27.0 * t1901 * t11556 * t26440 * t16011 - 2.0 / 27.0 * t1901 * t39167 * t26440 * t16155 - 4.0 / 27.0 * t1901 * t47399 * t26440 * t16160 + 10.0 / 81.0 * t1901 * t47799 * t102487 * t16150 + 8.0 / 27.0 * t11593 * t11556 * t26440 * t16169 + 2.0 / 3.0 * t446 * t452 * t5710 * t16093 - 2.0 / 9.0 * t117256 - t91625 + 2.0 / 3.0 * t1901 * t11472 * t26440 * t16150 - 4.0 / 9.0 * t1901 * t11556 * t103423 * t16150 - 8.0 / 9.0 * t11593 * t11472 * t26435 * t16169 + 4.0 / 27.0 * t117270 - 4.0 / 9.0 * t117272 - 4.0 / 9.0 * t117274;
    (t117276,)
}
