//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 739/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk739<F: Float>(t4458: F, t569: F, t616: F, t12752: F, t17041: F, t17045: F, t17049: F, t17053: F, t17057: F, t17060: F, t17063: F, t17068: F, t17073: F, t17078: F, t17083: F, t17088: F, t17091: F, t1901: F, t446: F) -> (F,) {
    let t17095 = t569 * t616 * t4458;
    let t17098 = 2.0 / 27.0 * t1901 * t17041 + 2.0 / 27.0 * t1901 * t17045 + 4.0 / 9.0 * t1901 * t17049 + 2.0 / 9.0 * t1901 * t17053 - 2.0 / 27.0 * t1901 * t17057 + t17060 / 9.0 - 2.0 / 3.0 * t446 * t17063 - 2.0 * t446 * t17068 - 2.0 / 3.0 * t446 * t17073 + 4.0 / 3.0 * t446 * t17078 - 2.0 / 3.0 * t446 * t17083 - 2.0 * t446 * t17088 - 2.0 / 9.0 * t17091 + 8.0 / 27.0 * t12752 + 2.0 / 9.0 * t446 * t17095;
    (t17098,)
}
