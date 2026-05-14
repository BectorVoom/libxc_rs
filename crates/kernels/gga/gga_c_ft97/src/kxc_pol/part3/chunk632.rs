//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 632/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk632<F: Float>(t157: F, t9224: F, t160: F, t7763: F, t7800: F, t1047: F, t1637: F, t89: F, t1570: F, t586: F, t1557: F, t1037: F, t1771: F, t3524: F, t458: F, t2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12723 = t9224 * t157;
    let t12724 = t160 * t7763;
    let t12746 = t160 * t7800;
    let t12752 = t89 * t1637 * t1047;
    let t12791 = t586 * t1570;
    let t12796 = t586 * t1557;
    let t12809 = t1771 * t1037;
    let t12816 = 2.0 / 3.0 * t458 * t3524;
    let t12823 = t9224 * t2;
    (t12723, t12724, t12746, t12752, t12791, t12796, t12809, t12816, t12823)
}
