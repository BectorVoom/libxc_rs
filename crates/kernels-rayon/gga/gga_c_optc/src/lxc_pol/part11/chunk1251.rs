//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1251/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1251(t22610: f64, t22728: f64, t56300: f64, t56301: f64, t56302: f64, t56303: f64, t56304: f64, t56305: f64, t56307: f64, t56308: f64, t56309: f64, t16939: f64, t3788: f64) -> (f64, f64) {
    let t56668 = t56300 - t56301 - t56302 + t22610 - t56303 - t56304 - t56305 + t22728 + t56307 - t56308 - t56309;
    let t56676 = 0.2077890707925103596e3_f64 * t3788 * t16939;
    (t56668, t56676)
}
