//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 390/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk390(t2120: f64, t579: f64, t91: f64, t1956: f64, t1959: f64, t1962: f64, t1967: f64, t1972: f64, t1977: f64, t1981: f64, t1989: f64, t2078: f64, t2089: f64) -> (f64, f64, f64) {
    let t2122 = t91 * t579 * t2120;
    let t2124 = 4.0_f64 / 27.0_f64 * t1956;
    let t2133 = -t2089 / 12.0_f64 + t2122 / 6.0_f64 + t2124 + 2.0_f64 / 27.0_f64 * t1959 + 2.0_f64 / 9.0_f64 * t1962 - 2.0_f64 / 27.0_f64 * t1967 + 2.0_f64 / 9.0_f64 * t1972 + 2.0_f64 / 9.0_f64 * t1977 - t1981 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t1989 - t2078 / 3.0_f64;
    (t2122, t2124, t2133)
}
