//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 376/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk376(t1985: f64, t1986: f64, t27: f64, t89: f64, t538: f64, t132: f64, t139: f64, t128: f64, t131: f64) -> (f64, f64, f64, f64, f64) {
    let t1987 = t1985 * t1986;
    let t1989 = t89 * t27 * t1987;
    let t1991 = t538 * t538;
    let t1992 = t1991 * t132;
    let t1993 = t1992 * t139;
    let t1995 = t128 * t131;
    (t1987, t1989, t1992, t1993, t1995)
}
