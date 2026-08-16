//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 897/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk897(t2482: f64, t2492: f64, t9267: f64, t4779: f64, t6574: f64, t584: f64) -> (f64, f64, f64, f64) {
    let t9268 = t2492 * t2482;
    let t9270 = 0.19171462976960374838e1_f64 * t9267 * t9268;
    let t9271 = t4779 * t6574;
    let t9272 = t584 * t9271;
    (t9268, t9270, t9271, t9272)
}
