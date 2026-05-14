//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 223/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk223<F: Float>(t192: F, t713: F, t743: F, t462: F, t736: F, t740: F, t92: F, t734: F, t91: F, t663: F, t672: F, t716: F) -> (F, F, F, F, F) {
    let t745 = t192 * t743 * t713;
    let t747 = -t736 - t462 * t740 / 3.0 - t92 * t745;
    let t749 = t91 * t734 * t747;
    let t751 = t663 / 9.0;
    let t754 = t749 / 6.0 - t751 - t672 / 9.0 - t716 / 3.0;
    (t745, t747, t749, t751, t754)
}
