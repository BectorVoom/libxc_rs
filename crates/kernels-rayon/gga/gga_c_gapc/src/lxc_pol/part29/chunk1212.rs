//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1212/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1212(t436: f64, t8775: f64, t8776: f64, t34940: f64, t34942: f64, t34946: f64, t34949: f64, t34951: f64, t34954: f64, t34956: f64, t34958: f64, t34960: f64, t34962: f64) -> f64 {
    let t34965 = t8775 * t436 * t8776;
    let t34967 = 0.80045999977926802213e-7_f64 * t34940 - 0.20259111355493285149e-5_f64 * t34942 + 0.88397049170382309323e-8_f64 * t34946 - 0.90579542097823505428e-7_f64 * t34949 - 0.25301920572916666668e-5_f64 * t34951 - 0.49190053374354708085e-8_f64 * t34954 - 0.13259557375557346398e-6_f64 * t34956 - 0.13259557375557346398e-6_f64 * t34958 - 0.6629778687778673199e-7_f64 * t34960 - 0.90579542097823505428e-7_f64 * t34962 - 0.22510123728325872388e-7_f64 * t34965;
    t34967
}
