//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 372/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk372<F: Float>(t3842: F, t729: F, t762: F, t1091: F, t724: F, t773: F, t265: F, t3746: F, t1175: F, t684: F, t1168: F, t713: F) -> (F, F, F, F, F) {
    let t3844 = t729 * t762 * t3842;
    let t3848 = t724 * t773 * t1091;
    let t3852 = t724 * t265 * t3746;
    let t3856 = t724 * t1175 * t684;
    let t3859 = t1168 * t713;
    (t3844, t3848, t3852, t3856, t3859)
}
