//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1442/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1442(t31744: f64, t7458: f64, t2314: f64, t33231: f64, t4034: f64, t1873: f64, t26870: f64, t652: f64, t4028: f64, t26114: f64, t8533: f64, t26179: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t122598 = t7458 * t31744;
    let t122599 = t2314 * t33231;
    let t122600 = t4034 * t33231;
    let t122602 = t652 * t26870 * t1873;
    let t122603 = t4028 * t31744;
    let t122604 = t26114 * t8533;
    let t122605 = t26179 * t8533;
    (t122598, t122599, t122600, t122602, t122603, t122604, t122605)
}
