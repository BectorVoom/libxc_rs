//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1064/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1064<F: Float>(t39926: F, t39970: F, t3928: F, t39911: F, t39915: F, t39917: F, t39921: F, t39932: F, t39934: F, t39940: F, t39946: F, t39951: F, t39954: F, t39956: F, t39964: F, t39966: F, t39968: F, t5226: F, t699: F) -> F {
    let t43190 = F::new(0.39726959900411316772e-4) * t39926;
    let t43204 = F::new(0.39726959900411316772e-4) * t39970;
    let t43205 = F::new(0.638468998399467591e-4) * t39911 - F::new(0.5107751987195740728e-4) * t39915 + F::new(0.5107751987195740728e-4) * t39917 - F::new(0.638468998399467591e-4) * t39921 - t43190 + F::new(0.1702583995731913576e-4) * t39932 - F::new(0.5107751987195740728e-4) * t39934 - F::new(0.5107751987195740728e-4) * t39940 + F::new(0.85129199786595678799e-5) * t39946 - F::new(0.638468998399467591e-4) * t39951 - F::new(0.13637330827122670865e-1) * t39954 - F::new(0.17961362552795712846e0) * t39956 + F::new(0.35922725105591425692e0) * t3928 * t699 * t5226 - F::new(0.15323255961587222184e-3) * t39964 - F::new(0.5107751987195740728e-4) * t39966 - F::new(0.85129199786595678799e-5) * t39968 + t43204;
    t43205
}
