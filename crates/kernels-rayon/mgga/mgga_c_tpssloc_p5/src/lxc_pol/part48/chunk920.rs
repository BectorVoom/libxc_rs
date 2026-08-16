//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 920/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk920(t641: f64, t31: f64, t607: f64, t645: f64, t79: f64, t8306: f64, t608: f64, t22633: f64, t22635: f64, t31090: f64, t90506: f64, t22642: f64, t22643: f64, t8458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113836 = t641 * t641;
    let t113864 = t645 * t31 * t607;
    let t113875 = t8306 * t79;
    let t113876 = t608 * t641;
    let t113931 = 0.13159472534785811492e0_f64 * t22633 * t22635 * t31090 * t90506;
    let t113934 = 0.16449340668482264365e-1_f64 * t22642 * t22643 * t8458;
    (t113836, t113864, t113875, t113876, t113931, t113934)
}
