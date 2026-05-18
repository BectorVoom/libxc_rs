//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1097/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1097<F: Float>(t30490: F, t30800: F, t46873: F, t46875: F, t46877: F, t46879: F, t46881: F, t46883: F, t46885: F, t46889: F, t46892: F, t46894: F, t46899: F, t46904: F, t46906: F, t46911: F, t46916: F, t46921: F, t739: F, t8041: F, t884: F) -> F {
    let t48818 = F::new(0.1702583995731913576e-4) * t46873 - F::new(0.1702583995731913576e-4) * t46875 - F::new(0.1702583995731913576e-4) * t46877 - F::new(0.1702583995731913576e-4) * t46879 + F::new(0.5107751987195740728e-4) * t46881 - F::new(0.15323255961587222184e-3) * t46883 + F::new(0.20431007948782962912e-3) * t46885 - F::new(0.10215503974391481456e-3) * t46889 + F::new(0.5107751987195740728e-4) * t46892 + F::new(0.638468998399467591e-4) * t46894 - F::new(0.5107751987195740728e-4) * t46899 - F::new(0.5107751987195740728e-4) * t46904 + F::new(0.5107751987195740728e-4) * t46906 + F::new(0.5107751987195740728e-4) * t46911 - F::new(0.5107751987195740728e-4) * t46916 - F::new(0.1702583995731913576e-4) * t46921 - F::new(0.35922725105591425692e0) * t739 * t8041 * t30800 + F::new(0.35922725105591425692e0) * t884 * t8041 * t30490;
    t48818
}
