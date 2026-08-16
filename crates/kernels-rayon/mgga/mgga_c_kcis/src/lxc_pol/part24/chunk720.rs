//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 720/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk720(t189: f64, t8823: f64, t2665: f64, t850: f64, t2683: f64, t851: f64, t47: f64, t8655: f64, t8656: f64, t8659: f64, t680: f64, t2372: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8824 = t189 * t8823;
    let t8825 = t2665 * t850;
    let t8826 = t8825 * t2683;
    let t8829 = t8825 * t851;
    let t8832 = t47 * t8655;
    let t8833 = t8656 * t8659;
    let t8836 = t8656 * t680;
    let t8845 = t88 * t2372;
    (t8824, t8825, t8826, t8829, t8832, t8833, t8836, t8845)
}
