//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1162/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1162(t13937: f64, t731: f64, t43173: f64, t43175: f64, t43179: f64, t43182: f64, t43185: f64, t43189: f64, t43190: f64, t43195: f64, t43196: f64, t43202: f64) -> f64 {
    let t47702 = t731 * t13937;
    let t47704 = t43173 + 0.92286314761706691403e-1_f64 * t43175 - t43179 + t43182 + t43185 - t43189 - t43190 - t43195 + 0.32043859292259267849e-3_f64 * t43196 - 0.42725145723012357132e-3_f64 * t47702 - t43202;
    t47704
}
