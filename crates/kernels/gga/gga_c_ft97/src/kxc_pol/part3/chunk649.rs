//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 649/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk649<F: Float>(t2360: F, t2923: F, t13540: F, t13542: F, t4317: F, t5: F, t1882: F, t4038: F, t4041: F, t4034: F, t4053: F, t4057: F, t681: F, t89: F, t10400: F, t10279: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14519 = t2923 * t2360;
    let t14544 = 0.6419148148148148148e-1 * t13540;
    let t14553 = 0.19257444444444444444e0 * t13542;
    let t14571 = t5 * t4317;
    let t14635 = t1882 * t4038;
    let t14636 = t14635 / 27.0;
    let t14637 = t1882 * t4041;
    let t14638 = 2.0 / 27.0 * t14637;
    let t14639 = t1882 * t4034;
    let t14640 = 2.0 / 81.0 * t14639;
    let t14657 = t1882 * t4053;
    let t14658 = t14657 / 27.0;
    let t14683 = t89 * t681 * t4057;
    let t14684 = 2.0 / 9.0 * t14683;
    let t14708 = 4.0 / 27.0 * t10400;
    let t14711 = 4.0 / 81.0 * t10279;
    (t14519, t14544, t14553, t14571, t14635, t14636, t14637, t14638, t14639, t14640, t14657, t14658, t14683, t14684, t14708, t14711)
}
