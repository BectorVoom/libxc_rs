//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 991/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk991(t23168: f64, t33376: f64, t33380: f64, t6579: f64, t1888: f64, t22996: f64, t26657: f64, t232: f64, t6646: f64, t7823: f64, t828: f64, t1880: f64, t1894: f64, t214: f64, t26653: f64) -> (f64, f64, f64, f64, f64) {
    let t121533 = t23168 * t33376;
    let t121536 = t6579 * t33380;
    let t121541 = t1888 * t22996 * t26657;
    let t121546 = t1888 * t6646 * t7823 * t828 * t232;
    let t121550 = t1880 * t214 * t1894 * t26653;
    (t121533, t121536, t121541, t121546, t121550)
}
