//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1203/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1203<F: Float>(t29799: F, t8392: F, t29948: F, t1882: F, t29896: F, t29853: F, t102678: F, t102682: F, t102862: F, t103200: F, t103211: F, t103216: F, t103219: F, t11810: F, t11902: F, t16266: F, t16313: F, t16320: F, t16324: F, t16328: F, t1901: F, t23339: F, t26357: F, t26423: F, t3238: F, t446: F, t452: F, t91771: F) -> (F,) {
    let t117566 = t8392 * t29799;
    let t117571 = t8392 * t29948;
    let t117573 = t1882 * t29896;
    let t117575 = t1882 * t29853;
    let t117597 = 4.0 / 9.0 * t117566 - t103200 + 2.0 / 9.0 * t1901 * t11902 * t26357 - 2.0 / 27.0 * t117571 + t117573 / 9.0 + t103211 + t103216 - t103219 + 2.0 / 27.0 * t117575 - 2.0 / 3.0 * t1901 * t11810 * t23339 * t16266 + 2.0 / 3.0 * t446 * t452 * t3238 * t26423 - 2.0 / 9.0 * t1901 * t91771 * t16320 - 4.0 / 9.0 * t1901 * t102862 * t16324 - 4.0 / 9.0 * t1901 * t102678 * t16328 + 4.0 / 27.0 * t1901 * t102682 * t16313;
    (t117597,)
}
