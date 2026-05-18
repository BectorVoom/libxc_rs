//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1147/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1147<F: Float>(t10379: F, t275: F, t10482: F, t1916: F, t2262: F, t42170: F, t42174: F, t44396: F, t44399: F, t44400: F, t47966: F, t47968: F, t47970: F, t47972: F, t47974: F, t47976: F, t47980: F, t47984: F, t47986: F, t47988: F) -> F {
    let t49771 = t275 * t10379;
    let t49772 = t275 * t10482;
    let t49787 = t49771 + t49772 + F::new(0.5107751987195740728e-4) * t47966 + F::new(0.1702583995731913576e-4) * t47968 - F::new(0.19957069503106347607e-1) * t1916 * t2262 - F::new(0.35922725105591425692e0) * t47970 + F::new(0.71845450211182851384e0) * t47972 + F::new(0.35922725105591425692e0) * t47974 + F::new(0.15965655602485078085e0) * t47976 - t44396 - F::new(0.2881692658299671676e-2) * t42170 + F::new(0.40992351065071538964e-3) * t42174 - t44399 - t44400 - F::new(0.15965655602485078085e0) * t47980 + F::new(0.47885174879960069324e-4) * t47984 + F::new(0.23942587439980034662e-4) * t47986 - F::new(0.2553875993597870364e-4) * t47988;
    t49787
}
