//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1180/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1180(t11848: f64, t11850: f64, t869: f64, t11854: f64, t7553: f64, t1078: f64, t2387: f64, t3756: f64, t11764: f64, t3427: f64, t11759: f64, t11761: f64) -> (f64, f64, f64, f64, f64) {
    let t33823 = t869 * t11848 * t11850;
    let t33825 = t7553 * t11854;
    let t33828 = t2387 * t3756 * t1078;
    let t33831 = t11764 * t3427;
    let t33834 = t869 * t11759 * t11761;
    (t33823, t33825, t33828, t33831, t33834)
}
