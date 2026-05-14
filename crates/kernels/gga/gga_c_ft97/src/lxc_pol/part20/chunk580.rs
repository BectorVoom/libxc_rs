//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 580/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk580<F: Float>(t8640: F, t895: F, t2253: F, t2934: F, t2920: F, t2941: F, t14: F, t7741: F, t12: F, t9: F) -> (F, F, F, F, F, F, F) {
    let t10921 = t8640 * t895;
    let t10923 = t2253 * t2934;
    let t10925 = t2253 * t2920;
    let t10927 = t2253 * t2941;
    let t11174 = 1.0 / t14 / t7741;
    let t11175 = t12 * t11174;
    let t11176 = t9 * t11175;
    (t10921, t10923, t10925, t10927, t11174, t11175, t11176)
}
