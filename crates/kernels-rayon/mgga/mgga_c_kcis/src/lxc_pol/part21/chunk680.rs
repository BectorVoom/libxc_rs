//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 680/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk680(t125: f64, t7603: f64, t86: f64, t7577: f64, t7584: f64, t7587: f64, t7593: f64, t7595: f64, t7598: f64, t7601: f64, t165: f64, t2146: f64, t782: f64) -> (f64, f64, f64, f64) {
    let t7605 = t86 * t125 * t7603;
    let t7607 = -0.69505208333333333333e-3_f64 * t7577 + 0.92754700520833333333e-4_f64 * t7584 + 0.16217881944444444444e-2_f64 * t7587 + 0.69505208333333333333e-3_f64 * t7593 + 0.69505208333333333333e-3_f64 * t7595 - 0.13265555555555555555e-1_f64 * t7598 + 0.99491666666666666664e-2_f64 * t7601 - 0.99491666666666666664e-2_f64 * t7605;
    let t7608 = t7607 * t165;
    let t7609 = t2146 * t782;
    (t7605, t7607, t7608, t7609)
}
