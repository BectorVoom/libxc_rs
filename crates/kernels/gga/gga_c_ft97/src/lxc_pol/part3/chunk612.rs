//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 612/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk612<F: Float>(t303: F, t3051: F, t1771: F, t854: F, t10491: F, t2: F, t10478: F, t305: F, t631: F, t7242: F, t798: F, t898: F, t10279: F, t10397: F, t192: F, t7640: F) -> (F, F, F, F, F, F, F, F) {
    let t10594 = 28.0 / 27.0 * t3051 * t303;
    let t10595 = t1771 * t854;
    let t10603 = t10491 * t2;
    let t10613 = t10478 * t2;
    let t10631 = 1.0 / t305 / t631 / t898 / t798 / t7242 / 4.0;
    let t10640 = 4.0 / 27.0 * t10279;
    let t10658 = 28.0 / 81.0 * t10397;
    let t10683 = t192 * t7640;
    (t10594, t10595, t10603, t10613, t10631, t10640, t10658, t10683)
}
