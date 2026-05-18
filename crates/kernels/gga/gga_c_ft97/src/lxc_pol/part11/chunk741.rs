//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 741/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk741<F: Float>(t241: F, t9568: F, t265: F, t9572: F, t2373: F, t766: F, t2574: F, t762: F, t2569: F, t713: F, t2568: F, t729: F) -> (F, F, F, F, F, F) {
    let t10024 = t9568 * t241;
    let t10026 = t10024 * t265 * t9572;
    let t10029 = t2373 * t766;
    let t10031 = t2574 * t762 * t10029;
    let t10034 = t2569 * t713;
    let t10036 = t729 * t2568 * t10034;
    (t10024, t10026, t10029, t10031, t10034, t10036)
}
