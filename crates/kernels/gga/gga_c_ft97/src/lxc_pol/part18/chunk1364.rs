//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1364/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1364<F: Float>(t1384: F, t7800: F, t27217: F, t8392: F, t27222: F, t104471: F, t105600: F, t11437: F, t12714: F, t12737: F, t13021: F, t13220: F, t144: F, t1643: F, t1651: F, t167: F, t1901: F, t2142: F, t2185: F, t23548: F, t26888: F, t26950: F, t26955: F, t27263: F, t379: F, t41269: F, t446: F, t574: F, t616: F, t6639: F, t6699: F, t9144: F, t9428: F) -> (F,) {
    let t106395 = t1384 * t7800;
    let t106413 = 4.0 / 27.0 * t8392 * t27217;
    let t106415 = 4.0 / 81.0 * t8392 * t27222;
    let t106451 = -4.0 / 9.0 * t1901 * t12714 * t106395 * t11437 - 2.0 / 9.0 * t1901 * t13220 * t23548 * t12737 - 2.0 / 9.0 * t1901 * t9144 * t27263 * t379 - t1901 * t9144 * t6639 * t1651 / 9.0 + t106413 - t106415 - t1901 * t9144 * t23548 * t13021 / 9.0 - t1901 * t9144 * t6699 * t1651 / 9.0 - 2.0 / 27.0 * t1901 * t41269 * t6699 * t1643 + t446 * t574 * t9428 * t6699 / 3.0 + 2.0 / 3.0 * t446 * t144 * t104471 + 2.0 / 3.0 * t446 * t574 * t2142 * t26955 + 4.0 / 3.0 * t446 * t2185 * t616 * t26950 + 4.0 / 3.0 * t446 * t2185 * t616 * t26888 + 4.0 / 3.0 * t446 * t2185 * t167 * t105600;
    (t106451,)
}
