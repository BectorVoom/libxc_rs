//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 459/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk459(t1411: f64, t941: f64, t1392: f64, t500: f64, t4066: f64, t4069: f64, t1535: f64, t446: f64, t4116: f64, t4120: f64, t4124: f64, t1004: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5321 = t941 * t1411;
    let t5372 = t500 * t1392;
    let t5375 = 48.0_f64 * t4066;
    let t5376 = 80.0_f64 * t4069;
    let t5377 = t1535 * t446;
    let t5382 = 12.0_f64 * t4116;
    let t5385 = 4.0_f64 * t4120;
    let t5388 = 32.0_f64 * t4124;
    let t5389 = t1004 * t589;
    (t5321, t5372, t5375, t5376, t5377, t5382, t5385, t5388, t5389)
}
