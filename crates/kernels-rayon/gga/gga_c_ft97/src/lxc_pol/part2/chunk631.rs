//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 631/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk631(t1711: f64, t371: f64, t407: f64, t391: f64, t625: f64, t68: f64, t72: f64, t2247: f64, t47: f64, t1675: f64, t172: f64, t173: f64, t1743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8042 = t371 * t1711;
    let t8050 = t407 * t407;
    let t8051 = 1.0_f64 / t8050;
    let t8074 = t68 * t391 * t625 * t72;
    let t8076 = t47 * t2247;
    let t8078 = t68 * t8076 * t72;
    let t8079 = 0.70937342644032921812e-2_f64 * t8078;
    let t8086 = t68 * t1675 * t172 * t72;
    let t8098 = t173 * t1743;
    (t8042, t8051, t8074, t8076, t8078, t8079, t8086, t8098)
}
