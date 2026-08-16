//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 399/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk399(t2142: f64, t609: f64, t144: f64, t1956: f64, t1959: f64, t1962: f64, t1967: f64, t1972: f64, t1977: f64, t1981: f64, t1989: f64, t2078: f64, t2089: f64, t2122: f64) -> (f64, f64, f64) {
    let t2143 = t2142 * t609;
    let t2144 = t144 * t2143;
    let t2149 = 4.0_f64 / 9.0_f64 * t1956;
    let t2157 = -t2089 / 4.0_f64 + t2122 / 2.0_f64 + t2149 + 2.0_f64 / 9.0_f64 * t1959 + 2.0_f64 / 3.0_f64 * t1962 - 2.0_f64 / 9.0_f64 * t1967 + 2.0_f64 / 3.0_f64 * t1972 + 2.0_f64 / 3.0_f64 * t1977 - t1981 / 3.0_f64 + 2.0_f64 * t1989 - t2078;
    (t2143, t2144, t2157)
}
