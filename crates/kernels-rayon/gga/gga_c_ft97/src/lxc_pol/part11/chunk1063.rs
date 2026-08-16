//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1063/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1063(t41891: f64, t41895: f64, t41898: f64, t41901: f64, t41905: f64, t41909: f64, t41915: f64, t41918: f64, t41922: f64, t41925: f64, t41927: f64, t41932: f64, t41935: f64, t41938: f64, t41942: f64) -> f64 {
    let t42042 = 8.0_f64 / 3.0_f64 * t41891 - 2.0_f64 / 9.0_f64 * t41895 + 16.0_f64 / 27.0_f64 * t41898 + 8.0_f64 / 9.0_f64 * t41901 + 8.0_f64 / 3.0_f64 * t41905 + 2.0_f64 / 3.0_f64 * t41909 - 80.0_f64 / 243.0_f64 * t41915 + 4.0_f64 / 27.0_f64 * t41918 - 8.0_f64 / 3.0_f64 * t41922 - 8.0_f64 / 3.0_f64 * t41925 + 16.0_f64 / 27.0_f64 * t41927 + 4.0_f64 / 9.0_f64 * t41932 + 8.0_f64 / 9.0_f64 * t41935 - 8.0_f64 / 27.0_f64 * t41938 + 8.0_f64 / 3.0_f64 * t41942;
    t42042
}
