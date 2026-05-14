//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1361/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1361<F: Float>(t1391: F, t2101: F, t582: F, t5935: F, t105357: F, t12338: F, t12645: F, t12710: F, t12945: F, t13166: F, t13220: F, t1643: F, t1901: F, t26526: F, t26999: F, t3424: F, t3429: F, t3446: F, t379: F, t40911: F, t41269: F, t49614: F, t51151: F, t5855: F, t5916: F, t5943: F, t6639: F, t9144: F, t95430: F, t95446: F, t95448: F, t95541: F, t95696: F) -> (F,) {
    let t106296 = t2101 * t1391;
    let t106300 = t582 * t5935;
    let t106311 = -2.0 / 9.0 * t1901 * t9144 * t95541 * t3424 - 4.0 / 9.0 * t1901 * t13220 * t95541 * t3429 + 2.0 / 3.0 * t1901 * t51151 * t105357 - 4.0 / 9.0 * t1901 * t13220 * t26526 * t379 - 2.0 / 27.0 * t1901 * t41269 * t6639 * t1643 + 2.0 / 9.0 * t1901 * t49614 * t5943 - 4.0 * t1901 * t26999 * t5855 * t12645 - 2.0 * t1901 * t26999 * t5855 * t12945 + 2.0 / 9.0 * t1901 * t95696 * t3446 - 4.0 / 9.0 * t1901 * t106296 * t12338 - 4.0 / 9.0 * t1901 * t106300 * t12710 - 2.0 / 9.0 * t95430 + 16.0 / 27.0 * t95446 + 8.0 / 27.0 * t95448 + 2.0 / 9.0 * t1901 * t40911 * t5916 * t13166;
    (t106311,)
}
