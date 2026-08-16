//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1151/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1151(t1008: f64, t6116: f64, t6121: f64, t6106: f64, t1016: f64, t1795: f64) -> (f64, f64, f64, f64) {
    let t20737 = t1008 * t6116;
    let t20739 = t1008 * t6121;
    let t20753 = t1008 * t6106;
    let t20764 = t1016 * t1795;
    (t20737, t20739, t20753, t20764)
}
