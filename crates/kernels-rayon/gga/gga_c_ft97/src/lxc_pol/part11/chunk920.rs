//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 920/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk920(t1906: f64, t38953: f64, t1786: f64, t1841: f64, t363: f64, t7745: f64, t110: f64, t1901: f64, t1902: f64, t1905: f64, t1909: f64, t3187: f64, t3194: f64, t37430: f64, t38921: f64, t38926: f64, t38928: f64, t38930: f64, t38935: f64, t38937: f64, t38942: f64, t38947: f64, t446: f64, t8210: f64, t8217: f64) -> (f64, f64) {
    let t38954 = t38953 * t1906;
    let t38956 = t1786 * t1841;
    let t38960 = t7745 * t363;
    let t38965 = 8.0_f64 * t446 * t38921 * t110 * t37430 + 16.0_f64 / 9.0_f64 * t38926 + 8.0_f64 / 3.0_f64 * t38928 - 4.0_f64 * t1901 * t1909 * t3194 * t38930 + 8.0_f64 / 9.0_f64 * t38935 + 8.0_f64 / 3.0_f64 * t1901 * t1909 * t8210 * t38937 + 8.0_f64 / 3.0_f64 * t1901 * t8217 * t3187 * t38942 + 8.0_f64 / 3.0_f64 * t1901 * t1902 * t3194 * t38947 + 16.0_f64 / 27.0_f64 * t38954 + 4.0_f64 / 3.0_f64 * t1901 * t38956 * t1905 + 8.0_f64 / 9.0_f64 * t1901 * t1909 * t3187 * t38960;
    (t38960, t38965)
}
