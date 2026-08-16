//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1401/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1401(t1657: f64, t3312: f64, t300: f64, t4832: f64, t14704: f64, t14710: f64, t14722: f64, t14781: f64, t14720: f64, t225: f64, t4947: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14850 = t1657 * t3312;
    let t14858 = t300 * t4832;
    let t14868 = 0.19931111111111111111e0_f64 * t14704;
    let t14870 = 0.10954222222222222222e0_f64 * t14710;
    let t14886 = 0.39862222222222222222e0_f64 * t14722;
    let t14890 = 0.21908444444444444444e0_f64 * t14781;
    let t14922 = 0.41203703703703703704e-2_f64 * t14720;
    let t14923 = 0.12361111111111111111e-1_f64 * t14722;
    let t14924 = 0.61805555555555555556e-2_f64 * t14704;
    let t14946 = 0.23744444444444444444e-1_f64 * t14722;
    let t14947 = 0.11872222222222222222e-1_f64 * t14704;
    let t14972 = t4947 * t225;
    (t14850, t14858, t14868, t14870, t14886, t14890, t14922, t14923, t14924, t14946, t14947, t14972)
}
