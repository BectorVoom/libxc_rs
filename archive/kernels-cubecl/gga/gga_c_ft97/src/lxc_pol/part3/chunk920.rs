//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 920/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk920<F: Float>(t1131: F, t3972: F, t729: F, t762: F, t4934: F, t713: F, t10157: F, t265: F, t5064: F, t2568: F, t766: F, t10052: F) -> (F, F, F, F) {
    let t18201 = t1131 * t3972;
    let t18203 = t729 * t762 * t18201;
    let t18206 = t4934 * t713;
    let t18208 = t10157 * t265 * t18206;
    let t18211 = t5064 * t713;
    let t18213 = t729 * t2568 * t18211;
    let t18216 = t5064 * t766;
    let t18217 = t10052 * t18216;
    (t18203, t18208, t18213, t18217)
}
