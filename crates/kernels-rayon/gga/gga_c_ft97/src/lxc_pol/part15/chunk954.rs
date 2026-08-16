//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 954/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk954(t1882: f64, t20546: f64, t1546: f64, t20667: f64, t89: f64, t20549: f64, t7780: f64, t1984: f64, t20655: f64, t20664: f64, t376: f64, t1775: f64, t20793: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78001 = t1882 * t20546;
    let t78012 = t89 * t1546 * t20667;
    let t78015 = t89 * t7780 * t20549;
    let t78017 = t1984 * t20655;
    let t78027 = t89 * t376 * t20664;
    let t78068 = t1775 * t20793;
    (t78001, t78012, t78015, t78017, t78027, t78068)
}
