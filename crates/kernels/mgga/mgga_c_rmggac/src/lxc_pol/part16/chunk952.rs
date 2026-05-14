//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 952/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk952<F: Float>(t2208: F, t2212: F, t31227: F, t32556: F, t37768: F, t43267: F, t43270: F, t43272: F, t43273: F, t43274: F, t45966: F, t45974: F, t45976: F, t45982: F, t45994: F, t45999: F, t46001: F, t46003: F, t46018: F, t46020: F) -> (F,) {
    let t48742 = -0.1702583995731913576e-4 * t45966 + 0.39914139006212695214e-1 * t32556 * t2212 + 0.59871208509319042821e-1 * t31227 * t2208 - t43267 - t43270 - 0.2553875993597870364e-4 * t45974 + 0.5107751987195740728e-4 * t45976 + 0.2553875993597870364e-4 * t45982 - t37768 + 0.1702583995731913576e-4 * t45994 - 0.47885174879960069325e-4 * t45999 + t43272 + t43273 + t43274 + 0.8980681276397856423e-1 * t46001 - 0.35922725105591425692e0 * t46003 - 0.1064114997332445985e-4 * t46018 - 0.5107751987195740728e-4 * t46020;
    (t48742,)
}
