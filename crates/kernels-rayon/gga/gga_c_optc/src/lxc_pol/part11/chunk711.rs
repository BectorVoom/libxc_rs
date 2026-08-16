//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 711/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk711(t256: f64, t7501: f64, t248: f64, t2516: f64, t243: f64, t7592: f64, t7523: f64, t808: f64, t251: f64, t2519: f64, t7341: f64, t224: f64, t2269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7753 = t256 * t7501;
    let t7758 = 1.0_f64 / t2516 / t248;
    let t7759 = t243 * t7758;
    let t7786 = 0.46308888888888888888e0_f64 * t7592;
    let t7787 = 0.16068111111111111111e1_f64 * t7523;
    let t7798 = 1.0_f64 / t2516 / t808;
    let t7799 = t243 * t7798;
    let t7801 = 1.0_f64 / t2519 / t251;
    let t7813 = t256 * t7341;
    let t7856 = 1.0_f64 / t224 / t2269;
    (t7753, t7758, t7759, t7786, t7787, t7798, t7799, t7801, t7813, t7856)
}
