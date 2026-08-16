//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 974/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk974(t34487: f64, t7380: f64, t1165: f64, t33509: f64, t604: f64, t7346: f64, t30468: f64, t4425: f64, t1470: f64, t30644: f64, t30984: f64, t8458: f64) -> (f64, f64, f64, f64, f64) {
    let t34488 = t7380 * t34487;
    let t34489 = 0.4584375e-1_f64 * t34488;
    let t34492 = t7346 * t1165 * t604 * t33509;
    let t34500 = t30468 * t4425;
    let t34501 = 0.34299214494455789578e-2_f64 * t34500;
    let t34506 = t30644 * t1470;
    let t34507 = 0.17149607247227894789e-2_f64 * t34506;
    let t34508 = t30984 * t8458;
    (t34489, t34492, t34501, t34507, t34508)
}
