//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 790/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk790<F: Float>(t13809: F, t13811: F, t10119: F, t13798: F, t13801: F, t13804: F, t13807: F, t13814: F, t13817: F, t13820: F, t13823: F, t13977: F, t13986: F, t13999: F, t762: F, t242: F) -> (F, F) {
    let t14004 = 2.0 / 9.0 * t13809;
    let t14005 = 4.0 / 9.0 * t13811;
    let t14010 = -2.0 / 9.0 * t13798 - 10.0 / 27.0 * t13801 + 8.0 / 9.0 * t13804 + t13807 / 3.0 - t14004 - t10119 - t14005 - 2.0 / 3.0 * t13814 - 2.0 * t13817 + 4.0 / 3.0 * t13820 - 2.0 / 3.0 * t13823;
    let t14012 = t13977 + t13986 + t13999 + t14010;
    let t14013 = t762 * t14012;
    let t14014 = t242 * t14013;
    (t14013, t14014)
}
