//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 631/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk631<F: Float>(t6947: F, t713: F, t729: F, t1175: F, t6061: F, t242: F, t27899: F, t14200: F, t27763: F, t14163: F, t27767: F, t684: F, t6861: F) -> (F, F, F, F, F, F) {
    let t28171 = t729 * t6947 * t713;
    let t28175 = t729 * t1175 * t6061;
    let t28178 = t242 * t27899;
    let t28181 = t14200 * t27763;
    let t28184 = t14163 * t27767;
    let t28187 = t6861 * t684;
    (t28171, t28175, t28178, t28181, t28184, t28187)
}
