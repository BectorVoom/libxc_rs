//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1164/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1164<F: Float>(t668: F, t7124: F, t10478: F, t1508: F, t25188: F, t848: F, t29190: F, t8392: F, t1882: F, t29170: F, t29378: F, t29120: F, t7126: F, t8232: F, t29116: F, t2842: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t112725 = t7124 * t668;
    let t112746 = t10478 * t1508;
    let t112760 = t848 * t25188;
    let t112765 = 4.0 / 27.0 * t8392 * t29190;
    let t112773 = 2.0 / 9.0 * t1882 * t29170;
    let t112775 = 2.0 / 9.0 * t1882 * t29378;
    let t112777 = 2.0 / 9.0 * t1882 * t29120;
    let t112778 = t8232 * t7126;
    let t112784 = 2.0 / 9.0 * t1882 * t29116;
    let t112785 = t2842 * t7124;
    (t112725, t112746, t112760, t112765, t112773, t112775, t112777, t112778, t112784, t112785)
}
