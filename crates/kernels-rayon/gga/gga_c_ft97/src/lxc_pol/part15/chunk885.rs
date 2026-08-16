//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 885/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk885(t41744: f64, t2937: f64, t326: f64, t2404: f64, t2680: f64, t683: f64, t7640: f64, t798: f64, t9568: f64, t295: f64, t41536: f64, t41446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43212 = 0.14978012345679012345e1_f64 * t41744;
    let t43250 = 1.0_f64 / t2937 / t326;
    let t43350 = t2404 * t2680;
    let t43381 = t683 * t7640;
    let t43468 = t9568 * t798;
    let t43480 = t295 * t41536;
    let t43495 = t295 * t41446;
    (t43212, t43250, t43350, t43381, t43468, t43480, t43495)
}
