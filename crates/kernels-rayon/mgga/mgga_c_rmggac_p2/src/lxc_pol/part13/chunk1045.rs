//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1045/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1045(t38934: f64, t38946: f64, t38965: f64, t38968: f64, t38986: f64, t34822: f64, t34826: f64, t38932: f64, t38938: f64, t38944: f64, t38958: f64, t38963: f64, t38971: f64, t38974: f64, t38978: f64, t38981: f64, t38984: f64, t38991: f64) -> f64 {
    let t42785 = 0.11918087970123395032e-3_f64 * t38934;
    let t42788 = 0.1454648621559751559e0_f64 * t38946;
    let t42793 = 0.66211599834018861287e-4_f64 * t38965;
    let t42794 = 0.49658699875514145965e-4_f64 * t38968;
    let t42800 = 0.11918087970123395032e-3_f64 * t38986;
    let t42802 = -0.2553875993597870364e-4_f64 * t38932 - t42785 - 0.20431007948782962912e-3_f64 * t38938 + 0.20431007948782962912e-3_f64 * t38944 + t42788 + 0.1454648621559751559e0_f64 * t34822 + 0.72732431077987577948e-1_f64 * t34826 + 0.85129199786595678799e-5_f64 * t38958 + 0.1702583995731913576e-4_f64 * t38963 - t42793 + t42794 + 0.5107751987195740728e-4_f64 * t38971 - 0.5454932330849068346e-1_f64 * t38974 - 0.35922725105591425692e0_f64 * t38978 - 0.23948483403727617128e0_f64 * t38981 + 0.35922725105591425692e0_f64 * t38984 - t42800 - 0.10215503974391481456e-3_f64 * t38991;
    t42802
}
