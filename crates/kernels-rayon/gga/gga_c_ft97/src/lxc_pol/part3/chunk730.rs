//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 730/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk730(t375: f64, t4130: f64, t89: f64, t14635: f64, t14637: f64, t14639: f64, t14657: f64, t14683: f64, t1775: f64, t4203: f64, t4207: f64, t4200: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14902 = t89 * t375 * t4130;
    let t14903 = t14902 / 9.0_f64;
    let t14921 = 2.0_f64 / 9.0_f64 * t14635;
    let t14922 = 4.0_f64 / 9.0_f64 * t14637;
    let t14923 = 4.0_f64 / 27.0_f64 * t14639;
    let t14929 = 2.0_f64 / 9.0_f64 * t14657;
    let t14936 = 4.0_f64 / 3.0_f64 * t14683;
    let t14951 = 2.0_f64 / 3.0_f64 * t14902;
    let t14953 = 2.0_f64 / 9.0_f64 * t1775 * t4203;
    let t14955 = 4.0_f64 / 9.0_f64 * t1775 * t4207;
    let t14957 = 4.0_f64 / 27.0_f64 * t1775 * t4200;
    (t14902, t14903, t14921, t14922, t14923, t14929, t14936, t14951, t14953, t14955, t14957)
}
