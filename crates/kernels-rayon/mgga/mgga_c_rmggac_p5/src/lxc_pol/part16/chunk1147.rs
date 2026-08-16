//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1147/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1147(t10379: f64, t275: f64, t10482: f64, t1916: f64, t2262: f64, t42170: f64, t42174: f64, t44396: f64, t44399: f64, t44400: f64, t47966: f64, t47968: f64, t47970: f64, t47972: f64, t47974: f64, t47976: f64, t47980: f64, t47984: f64, t47986: f64, t47988: f64) -> f64 {
    let t49771 = t275 * t10379;
    let t49772 = t275 * t10482;
    let t49787 = t49771 + t49772 + 0.5107751987195740728e-4_f64 * t47966 + 0.1702583995731913576e-4_f64 * t47968 - 0.19957069503106347607e-1_f64 * t1916 * t2262 - 0.35922725105591425692e0_f64 * t47970 + 0.71845450211182851384e0_f64 * t47972 + 0.35922725105591425692e0_f64 * t47974 + 0.15965655602485078085e0_f64 * t47976 - t44396 - 0.2881692658299671676e-2_f64 * t42170 + 0.40992351065071538964e-3_f64 * t42174 - t44399 - t44400 - 0.15965655602485078085e0_f64 * t47980 + 0.47885174879960069324e-4_f64 * t47984 + 0.23942587439980034662e-4_f64 * t47986 - 0.2553875993597870364e-4_f64 * t47988;
    t49787
}
