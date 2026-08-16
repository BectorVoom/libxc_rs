//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 783/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk783(t12020: f64, t16150: f64, t3193: f64, t432: f64, t4417: f64, t3187: f64, t1902: f64, t492: f64, t8424: f64, t1909: f64, t3194: f64, t18: f64, t920: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16151 = t12020 * t16150;
    let t16152 = t3193 * t16151;
    let t16155 = t4417 * t432;
    let t16156 = t3187 * t16155;
    let t16157 = t1902 * t16156;
    let t16160 = t4417 * t492;
    let t16161 = t8424 * t16160;
    let t16162 = t1909 * t16161;
    let t16165 = t3194 * t16150;
    let t16166 = t1909 * t16165;
    let t16169 = t920 * t18;
    (t16152, t16155, t16157, t16160, t16162, t16166, t16169)
}
