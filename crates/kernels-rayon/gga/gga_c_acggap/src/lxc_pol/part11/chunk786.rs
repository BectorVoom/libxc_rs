//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 786/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk786(t1535: f64, t8511: f64, t1345: f64, t604: f64, t1181: f64, t7575: f64, t2263: f64, t4680: f64, t2068: f64, t1411: f64, t599: f64, t1983: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8512 = t8511 * t1535;
    let t8514 = t604 * t1345;
    let t8515 = t1181 * t8514;
    let t8516 = t7575 * t8515;
    let t8518 = t4680 * t2263;
    let t8519 = t2068 * t8518;
    let t8521 = t599 * t1411;
    let t8522 = t1181 * t8521;
    let t8523 = t2068 * t8522;
    let t8525 = t1983 * t525;
    (t8512, t8514, t8515, t8516, t8518, t8519, t8521, t8522, t8523, t8525)
}
