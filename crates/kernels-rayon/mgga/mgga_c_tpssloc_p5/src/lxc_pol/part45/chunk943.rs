//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 943/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk943(t608: f64, t641: f64, t31005: f64, t6504: f64, t8513: f64, t79: f64, t22633: f64, t22635: f64, t31090: f64, t90506: f64, t22642: f64, t22643: f64, t8458: f64) -> (f64, f64, f64, f64, f64) {
    let t113876 = t608 * t641;
    let t113890 = t8513 * t31005 * t6504;
    let t113907 = t8513 * t79 * t6504 * t641;
    let t113931 = 0.13159472534785811492e0_f64 * t22633 * t22635 * t31090 * t90506;
    let t113934 = 0.16449340668482264365e-1_f64 * t22642 * t22643 * t8458;
    (t113876, t113890, t113907, t113931, t113934)
}
