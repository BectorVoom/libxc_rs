//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1019/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1019(t31603: f64, t1588: f64, t7614: f64, t1988: f64, t8855: f64, t7799: f64, t8859: f64, t1488: f64, t1980: f64, t1982: f64, t1983: f64, t30318: f64, t537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35812 = 13.0_f64 / 144.0_f64 * t31603;
    let t35814 = t7614 * t1588;
    let t35816 = t1988 * t8855;
    let t35818 = t7799 * t8859;
    let t35827 = t1980 * t1982 * t1488 * t1983;
    let t35829 = t30318 * t537;
    (t35812, t35814, t35816, t35818, t35827, t35829)
}
