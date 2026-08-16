//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 382/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk382(t2071: f64, t550: f64, t133: f64, t1355: f64, t140: f64, t1683: f64, t1698: f64, t1993: f64, t1996: f64, t2001: f64, t2003: f64, t2032: f64, t2036: f64, t2038: f64, t2043: f64, t2045: f64, t2060: f64, t399: f64, t540: f64, t543: f64) -> f64 {
    let t2072 = t550 * t2071;
    let t2074 = 2.0_f64 * t1993 - 0.2416365355361531912e1_f64 * t1996 * t399 + 0.2416365355361531912e1_f64 * t540 * t399 - 4.0_f64 * t2001 * t2003 + 2.0_f64 * t2032 + 0.72985269132393279984e0_f64 * t2036 * t2038 - 0.29194107652957311994e1_f64 * t543 * t1698 + 0.1208182677680765956e1_f64 * t2043 * t2045 + 0.38259118126557588605e1_f64 * t543 * t1683 + 0.14597053826478655997e1_f64 * t140 * t1698 - 0.1208182677680765956e1_f64 * t1355 * t2045 - 0.38259118126557588605e1_f64 * t140 * t1683 + 2.0_f64 * t133 * t2060 - t133 * t2072;
    t2074
}
