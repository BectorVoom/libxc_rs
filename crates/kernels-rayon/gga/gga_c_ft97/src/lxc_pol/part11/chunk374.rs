//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 374/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk374(t379: f64, t558: f64, t1969: f64, t446: f64, t143: f64, t1570: f64, t1559: f64, t356: f64, t89: f64, t1580: f64, t519: f64, t142: f64, t524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1970 = t379 * t558;
    let t1971 = t1969 * t1970;
    let t1972 = t446 * t1971;
    let t1974 = t143 * t1570;
    let t1975 = t1974 * t1559;
    let t1977 = t89 * t356 * t1975;
    let t1979 = t519 * t1580;
    let t1981 = t89 * t356 * t1979;
    let t1983 = t524 * t142;
    (t1970, t1971, t1972, t1974, t1975, t1977, t1979, t1981, t1983)
}
