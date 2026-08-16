//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1055/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1055(t2356: f64, t8232: f64, t41877: f64, t41882: f64, t41886: f64, t41891: f64, t41895: f64, t41899: f64, t41901: f64, t41905: f64, t41909: f64, t41915: f64, t41918: f64, t41922: f64, t41925: f64) -> (f64, f64) {
    let t41927 = t8232 * t2356;
    let t41928 = 8.0_f64 / 27.0_f64 * t41927;
    let t41929 = -2.0_f64 / 9.0_f64 * t41877 - 4.0_f64 / 9.0_f64 * t41882 - 8.0_f64 / 9.0_f64 * t41886 + 4.0_f64 / 3.0_f64 * t41891 - t41895 / 9.0_f64 + t41899 + 4.0_f64 / 9.0_f64 * t41901 + 4.0_f64 / 3.0_f64 * t41905 + t41909 / 3.0_f64 - 40.0_f64 / 243.0_f64 * t41915 + 2.0_f64 / 27.0_f64 * t41918 - 4.0_f64 / 3.0_f64 * t41922 - 4.0_f64 / 3.0_f64 * t41925 + t41928;
    (t41927, t41929)
}
