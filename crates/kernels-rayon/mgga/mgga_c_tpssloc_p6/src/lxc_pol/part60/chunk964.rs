//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 964/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk964(t118472: f64, t1484: f64, t22986: f64, t23270: f64, t112899: f64, t28267: f64, t118821: f64, t1527: f64, t1888: f64, t1880: f64, t28263: f64, t30663: f64) -> (f64, f64, f64, f64) {
    let t126233 = 0.6579736267392905746e-1_f64 * t22986 * t23270 * t118472 * t1484;
    let t126240 = 0.6579736267392905746e-1_f64 * t22986 * t112899 * t28267;
    let t126246 = 0.6579736267392905746e-1_f64 * t1888 * t23270 * t118821 * t1527;
    let t126249 = 0.16449340668482264365e-1_f64 * t1880 * t30663 * t28263;
    (t126233, t126240, t126246, t126249)
}
