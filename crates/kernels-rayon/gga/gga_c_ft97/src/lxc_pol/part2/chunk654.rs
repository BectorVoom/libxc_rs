//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 654/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk654(t1771: f64, t588: f64, t1775: f64, t2103: f64, t2106: f64, t2: f64, t9114: f64, t583: f64, t8282: f64, t2109: f64, t2098: f64, t2114: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9179 = t1771 * t588;
    let t9188 = t1775 * t2103;
    let t9190 = t1775 * t2106;
    let t9192 = t9114 * t2;
    let t9202 = t8282 * t583;
    let t9205 = t1775 * t2109;
    let t9207 = t1775 * t2098;
    let t9209 = t458 * t2114;
    (t9179, t9188, t9190, t9192, t9202, t9205, t9207, t9209)
}
