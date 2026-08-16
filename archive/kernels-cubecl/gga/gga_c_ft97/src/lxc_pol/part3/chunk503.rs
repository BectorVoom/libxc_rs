//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 503/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk503<F: Float>(t265: F, t3746: F, t724: F, t1175: F, t684: F, t1168: F, t713: F, t729: F, t762: F, t766: F, t2568: F, t242: F) -> (F, F, F, F, F, F, F) {
    let t3852 = t724 * t265 * t3746;
    let t3856 = t724 * t1175 * t684;
    let t3859 = t1168 * t713;
    let t3861 = t729 * t762 * t3859;
    let t3864 = t1168 * t766;
    let t3865 = t2568 * t3864;
    let t3866 = t242 * t3865;
    (t3852, t3856, t3859, t3861, t3864, t3865, t3866)
}
