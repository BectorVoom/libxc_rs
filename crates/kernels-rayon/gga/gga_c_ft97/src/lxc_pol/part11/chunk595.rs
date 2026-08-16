//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 595/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk595(t466: f64, t8282: f64, t1775: f64, t1797: f64, t1783: f64, t1802: f64, t458: f64, t2: f64, t8216: f64, t7825: f64, t1787: f64, t7829: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8283 = t8282 * t466;
    let t8285 = t1775 * t1797;
    let t8287 = t1775 * t1783;
    let t8289 = t458 * t1802;
    let t8291 = t8216 * t2;
    let t8292 = t8291 * t7825;
    let t8295 = t1787 * t7829;
    (t8283, t8285, t8287, t8289, t8291, t8292, t8295)
}
