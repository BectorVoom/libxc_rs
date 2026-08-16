//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1026/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1026(t1181: f64, t19834: f64, t2068: f64, t599: f64, t1983: f64, t30262: f64, t7586: f64, t8406: f64, t4680: f64, t7346: f64, t8896: f64, t7433: f64, t8962: f64) -> (f64, f64, f64, f64) {
    let t34123 = t2068 * t1181 * t599 * t19834;
    let t34127 = t30262 * t7586 * t1983 * t8406;
    let t34130 = t7346 * t4680 * t8896;
    let t34131 = 0.21437009059034868486e-3_f64 * t34130;
    let t34132 = t7433 * t8962;
    (t34123, t34127, t34131, t34132)
}
