//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1408/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1408<F: Float>(t1657: F, t3312: F, t300: F, t4832: F, t14704: F, t14710: F, t14722: F, t14781: F, t14720: F, t225: F, t4947: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14850 = t1657 * t3312;
    let t14858 = t300 * t4832;
    let t14868 = F::cast_from(0.19931111111111111111e0_f64) * t14704;
    let t14870 = F::cast_from(0.10954222222222222222e0_f64) * t14710;
    let t14886 = F::cast_from(0.39862222222222222222e0_f64) * t14722;
    let t14890 = F::cast_from(0.21908444444444444444e0_f64) * t14781;
    let t14922 = F::cast_from(0.41203703703703703704e-2_f64) * t14720;
    let t14923 = F::cast_from(0.12361111111111111111e-1_f64) * t14722;
    let t14924 = F::cast_from(0.61805555555555555556e-2_f64) * t14704;
    let t14946 = F::cast_from(0.23744444444444444444e-1_f64) * t14722;
    let t14947 = F::cast_from(0.11872222222222222222e-1_f64) * t14704;
    let t14972 = t4947 * t225;
    (t14850, t14858, t14868, t14870, t14886, t14890, t14922, t14923, t14924, t14946, t14947, t14972)
}
