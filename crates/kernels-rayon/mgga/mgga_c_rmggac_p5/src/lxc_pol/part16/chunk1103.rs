//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1103/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1103(t10270: f64, t2604: f64, t37904: f64, t40623: f64, t43654: f64, t47156: f64, t47158: f64, t47162: f64, t47167: f64, t47173: f64, t47175: f64, t47178: f64, t47180: f64, t47182: f64, t47188: f64, t6434: f64, t6449: f64, t6522: f64, t699: f64, t739: f64, t8264: f64, t903: f64) -> f64 {
    let t48924 = 0.20431007948782962912e-3_f64 * t47156 + 0.10215503974391481456e-3_f64 * t47158 + 0.5107751987195740728e-4_f64 * t47162 - 0.5107751987195740728e-4_f64 * t47167 + 0.3405167991463827152e-4_f64 * t47173 - 0.58540737209111952978e0_f64 * t40623 - 0.23948483403727617128e0_f64 * t47175 + 0.23948483403727617128e0_f64 * t739 * t8264 * t6522 + 0.35922725105591425692e0_f64 * t903 * t699 * t6449 + 0.35922725105591425692e0_f64 * t903 * t699 * t6434 + 0.23948483403727617128e0_f64 * t2604 * t10270 + 0.39726959900411316773e-4_f64 * t47178 + 0.39726959900411316773e-4_f64 * t47180 + t43654 + 0.11918087970123395032e-3_f64 * t47182 + 0.15965655602485078085e0_f64 * t47188 + t37904;
    t48924
}
