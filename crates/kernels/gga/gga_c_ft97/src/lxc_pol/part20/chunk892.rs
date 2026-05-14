//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 892/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk892<F: Float>(t28128: F, t3864: F, t14127: F, t14159: F, t6075: F, t24737: F, t3859: F, t13885: F, t255: F, t9707: F) -> (F, F, F, F, F, F) {
    let t28129 = t28128 * t3864;
    let t28130 = t14127 * t28129;
    let t28133 = t14159 * t6075;
    let t28136 = t24737 * t3859;
    let t28137 = t13885 * t28136;
    let t28140 = t9707 * t255;
    (t28129, t28130, t28133, t28136, t28137, t28140)
}
