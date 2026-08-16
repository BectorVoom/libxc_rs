//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 947/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk947(t11129: f64, t1156: f64, t3400: f64, t1164: f64, t268: f64, t405: f64, t6546: f64) -> (f64, f64, f64) {
    let t11131 = t3400 * t11129 * t1156;
    let t11133 = 0.35089341735807877242e1_f64 * t1164 * t11131;
    let t11135 = t268 * t6546 * t405;
    (t11131, t11133, t11135)
}
