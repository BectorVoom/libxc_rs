//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1093/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1093(t2208: f64, t2212: f64, t31227: f64, t32556: f64, t37768: f64, t43267: f64, t43270: f64, t43272: f64, t43273: f64, t43274: f64, t45966: f64, t45974: f64, t45976: f64, t45982: f64, t45994: f64, t45999: f64, t46001: f64, t46003: f64, t46018: f64, t46020: f64) -> f64 {
    let t48742 = -0.1702583995731913576e-4_f64 * t45966 + 0.39914139006212695214e-1_f64 * t32556 * t2212 + 0.59871208509319042821e-1_f64 * t31227 * t2208 - t43267 - t43270 - 0.2553875993597870364e-4_f64 * t45974 + 0.5107751987195740728e-4_f64 * t45976 + 0.2553875993597870364e-4_f64 * t45982 - t37768 + 0.1702583995731913576e-4_f64 * t45994 - 0.47885174879960069325e-4_f64 * t45999 + t43272 + t43273 + t43274 + 0.8980681276397856423e-1_f64 * t46001 - 0.35922725105591425692e0_f64 * t46003 - 0.1064114997332445985e-4_f64 * t46018 - 0.5107751987195740728e-4_f64 * t46020;
    t48742
}
