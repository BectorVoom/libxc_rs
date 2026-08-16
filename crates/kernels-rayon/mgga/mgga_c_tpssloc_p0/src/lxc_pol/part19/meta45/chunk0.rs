//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 304/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk304(t894: f64, t896: f64, t880: f64, t273: f64, t241: f64, t697: f64, t281: f64, t283: f64, t340: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t897 = t894 * t896;
    let t899 = 0.29896666666666666667e0_f64 * t880;
    let t901 = f64::sqrt(t273);
    let t902 = t901 * t896;
    let t904 = t697 * t241;
    let t906 = t281 * t904 * t283;
    let t907 = 0.82156666666666666667e-1_f64 * t906;
    let t908 = t241 * t340;
    (t897, t899, t901, t902, t904, t906, t907, t908)
}
