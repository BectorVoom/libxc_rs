//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1064/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1064(t39926: f64, t39970: f64, t3928: f64, t39911: f64, t39915: f64, t39917: f64, t39921: f64, t39932: f64, t39934: f64, t39940: f64, t39946: f64, t39951: f64, t39954: f64, t39956: f64, t39964: f64, t39966: f64, t39968: f64, t5226: f64, t699: f64) -> f64 {
    let t43190 = 0.39726959900411316772e-4_f64 * t39926;
    let t43204 = 0.39726959900411316772e-4_f64 * t39970;
    let t43205 = 0.638468998399467591e-4_f64 * t39911 - 0.5107751987195740728e-4_f64 * t39915 + 0.5107751987195740728e-4_f64 * t39917 - 0.638468998399467591e-4_f64 * t39921 - t43190 + 0.1702583995731913576e-4_f64 * t39932 - 0.5107751987195740728e-4_f64 * t39934 - 0.5107751987195740728e-4_f64 * t39940 + 0.85129199786595678799e-5_f64 * t39946 - 0.638468998399467591e-4_f64 * t39951 - 0.13637330827122670865e-1_f64 * t39954 - 0.17961362552795712846e0_f64 * t39956 + 0.35922725105591425692e0_f64 * t3928 * t699 * t5226 - 0.15323255961587222184e-3_f64 * t39964 - 0.5107751987195740728e-4_f64 * t39966 - 0.85129199786595678799e-5_f64 * t39968 + t43204;
    t43205
}
