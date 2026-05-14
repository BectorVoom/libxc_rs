//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 829/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk829<F: Float>(t1906: F, t38953: F, t1786: F, t1841: F, t363: F, t7745: F, t110: F, t1901: F, t1902: F, t1905: F, t1909: F, t3187: F, t3194: F, t37430: F, t38921: F, t38926: F, t38928: F, t38930: F, t38935: F, t38937: F, t38942: F, t38947: F, t446: F, t8210: F, t8217: F) -> (F, F) {
    let t38954 = t38953 * t1906;
    let t38956 = t1786 * t1841;
    let t38960 = t7745 * t363;
    let t38965 = 8.0 * t446 * t38921 * t110 * t37430 + 16.0 / 9.0 * t38926 + 8.0 / 3.0 * t38928 - 4.0 * t1901 * t1909 * t3194 * t38930 + 8.0 / 9.0 * t38935 + 8.0 / 3.0 * t1901 * t1909 * t8210 * t38937 + 8.0 / 3.0 * t1901 * t8217 * t3187 * t38942 + 8.0 / 3.0 * t1901 * t1902 * t3194 * t38947 + 16.0 / 27.0 * t38954 + 4.0 / 3.0 * t1901 * t38956 * t1905 + 8.0 / 9.0 * t1901 * t1909 * t3187 * t38960;
    (t38960, t38965)
}
