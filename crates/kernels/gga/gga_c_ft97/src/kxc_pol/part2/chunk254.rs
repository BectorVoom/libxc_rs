//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 254/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk254<F: Float>(t898: F, t900: F, t904: F, t631: F, t892: F, t895: F, t332: F, t113: F, t19: F, t362: F) -> (F, F, F, F, F) {
    let t906 = t898 * t900 * t904;
    let t909 = t892 + t631 * t895 / 6.0 + t631 * t906 / 2.0;
    let t910 = t909 * t332;
    let t911 = t910 * t113;
    let t920 = -t19 - t362;
    (t906, t909, t910, t911, t920)
}
