//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 792/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk792<F: Float>(t432: F, t4589: F, t452: F, t488: F, t492: F, t1852: F, t83: F, t1882: F, t4574: F, t4565: F, t4561: F, t4557: F) -> (F, F, F, F, F, F, F) {
    let t16286 = t4589 * t432;
    let t16288 = t452 * t488 * t16286;
    let t16291 = t4589 * t492;
    let t16292 = t1852 * t16291;
    let t16293 = t83 * t16292;
    let t16296 = t1882 * t4574;
    let t16298 = t1882 * t4565;
    let t16300 = t1882 * t4561;
    let t16302 = t1882 * t4557;
    (t16288, t16292, t16293, t16296, t16298, t16300, t16302)
}
