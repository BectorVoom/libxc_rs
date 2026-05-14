//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 973/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk973<F: Float>(t1526: F, t9483: F, t9503: F, t13598: F, t9491: F, t9499: F, t342: F, t657: F, t8639: F, t2252: F, t2326: F, t630: F, t9507: F, t762: F, t9895: F, t2492: F, t2568: F) -> (F, F, F, F, F, F, F, F) {
    let t42270 = t1526 * t9483 * t9503;
    let t42273 = t1526 * t13598 * t9491;
    let t42288 = t1526 * t9483 * t9499;
    let t42293 = 5.0 / 54.0 * t342 * t8639 * t657;
    let t42295 = t342 * t2252 * t2326;
    let t42320 = t342 * t630 * t9507;
    let t42334 = t9895 * t762;
    let t42339 = t2492 * t2568;
    (t42270, t42273, t42288, t42293, t42295, t42320, t42334, t42339)
}
