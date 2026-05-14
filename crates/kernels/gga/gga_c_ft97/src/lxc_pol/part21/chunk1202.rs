//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1202/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1202<F: Float>(t103: F, t29569: F, t102848: F, t103121: F, t103142: F, t103195: F, t103198: F, t110: F, t116595: F, t11810: F, t16120: F, t16233: F, t1901: F, t1902: F, t26061: F, t26154: F, t26171: F, t29721: F, t29922: F, t3200: F, t3238: F, t3271: F, t379: F, t446: F, t452: F, t4572: F, t4607: F, t47007: F, t499: F, t5630: F, t8411: F, t91583: F, t92035: F, t92049: F) -> (F,) {
    let t117521 = t103 * t29569;
    let t117564 = t1901 * t1902 * t117521 * t379 / 9.0 - 8.0 / 27.0 * t103121 + 2.0 / 27.0 * t1901 * t92035 * t16233 + 2.0 / 9.0 * t1901 * t92049 * t4607 - 4.0 * t1901 * t26171 * t5630 * t16120 - t103142 + 2.0 / 9.0 * t1901 * t102848 * t3200 - 4.0 / 3.0 * t1901 * t47007 * t29922 - 4.0 / 3.0 * t1901 * t11810 * t91583 * t4572 + 8.0 / 27.0 * t103195 + 2.0 / 3.0 * t446 * t452 * t3238 * t26154 - 2.0 * t446 * t8411 * t499 * t29721 - 2.0 * t446 * t8411 * t110 * t116595 + 2.0 / 3.0 * t446 * t452 * t26061 * t3271 + t103198;
    (t117564,)
}
