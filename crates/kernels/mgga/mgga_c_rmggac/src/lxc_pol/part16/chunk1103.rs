//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1103/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1103<F: Float>(t10270: F, t2604: F, t37904: F, t40623: F, t43654: F, t47156: F, t47158: F, t47162: F, t47167: F, t47173: F, t47175: F, t47178: F, t47180: F, t47182: F, t47188: F, t6434: F, t6449: F, t6522: F, t699: F, t739: F, t8264: F, t903: F) -> F {
    let t48924 = F::new(0.20431007948782962912e-3) * t47156 + F::new(0.10215503974391481456e-3) * t47158 + F::new(0.5107751987195740728e-4) * t47162 - F::new(0.5107751987195740728e-4) * t47167 + F::new(0.3405167991463827152e-4) * t47173 - F::new(0.58540737209111952978e0) * t40623 - F::new(0.23948483403727617128e0) * t47175 + F::new(0.23948483403727617128e0) * t739 * t8264 * t6522 + F::new(0.35922725105591425692e0) * t903 * t699 * t6449 + F::new(0.35922725105591425692e0) * t903 * t699 * t6434 + F::new(0.23948483403727617128e0) * t2604 * t10270 + F::new(0.39726959900411316773e-4) * t47178 + F::new(0.39726959900411316773e-4) * t47180 + t43654 + F::new(0.11918087970123395032e-3) * t47182 + F::new(0.15965655602485078085e0) * t47188 + t37904;
    t48924
}
