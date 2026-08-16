//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 720/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk720(t16891: f64, t4699: f64, t1014: f64, t4674: f64, t12401: f64, t4702: f64, t1013: f64, t16907: f64, t3355: f64, t4710: f64, t19977: f64, t8690: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20576 = t16891 * t4699;
    let t20578 = t4674 * t1014;
    let t20580 = t12401 * t4702;
    let t20583 = t16907 * t1013;
    let t20586 = t3355 * t4710;
    let t20589 = t8690 * t19977;
    (t20576, t20578, t20580, t20583, t20586, t20589)
}
