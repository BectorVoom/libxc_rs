//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1045/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1045<F: Float>(t38934: F, t38946: F, t38965: F, t38968: F, t38986: F, t34822: F, t34826: F, t38932: F, t38938: F, t38944: F, t38958: F, t38963: F, t38971: F, t38974: F, t38978: F, t38981: F, t38984: F, t38991: F) -> F {
    let t42785 = F::new(0.11918087970123395032e-3) * t38934;
    let t42788 = F::new(0.1454648621559751559e0) * t38946;
    let t42793 = F::new(0.66211599834018861287e-4) * t38965;
    let t42794 = F::new(0.49658699875514145965e-4) * t38968;
    let t42800 = F::new(0.11918087970123395032e-3) * t38986;
    let t42802 = -F::new(0.2553875993597870364e-4) * t38932 - t42785 - F::new(0.20431007948782962912e-3) * t38938 + F::new(0.20431007948782962912e-3) * t38944 + t42788 + F::new(0.1454648621559751559e0) * t34822 + F::new(0.72732431077987577948e-1) * t34826 + F::new(0.85129199786595678799e-5) * t38958 + F::new(0.1702583995731913576e-4) * t38963 - t42793 + t42794 + F::new(0.5107751987195740728e-4) * t38971 - F::new(0.5454932330849068346e-1) * t38974 - F::new(0.35922725105591425692e0) * t38978 - F::new(0.23948483403727617128e0) * t38981 + F::new(0.35922725105591425692e0) * t38984 - t42800 - F::new(0.10215503974391481456e-3) * t38991;
    t42802
}
