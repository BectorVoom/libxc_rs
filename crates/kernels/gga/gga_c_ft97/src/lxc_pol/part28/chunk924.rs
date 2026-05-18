//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 924/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk924<F: Float>(t1339: F, t8326: F, t488: F, t7750: F, t23339: F, t47667: F, t370: F, t8418: F, t26166: F, t463: F, t6524: F, t165: F, t26768: F) -> (F, F, F, F, F, F, F) {
    let t103472 = t8326 * t1339;
    let t103491 = t7750 * t488;
    let t103510 = t47667 * t23339;
    let t103626 = t370 * t8418;
    let t103654 = t47667 * t26166;
    let t103823 = t463 * t6524;
    let t104205 = t26768 * t165;
    (t103472, t103491, t103510, t103626, t103654, t103823, t104205)
}
