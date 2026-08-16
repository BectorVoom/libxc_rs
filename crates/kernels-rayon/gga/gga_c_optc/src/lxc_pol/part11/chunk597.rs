//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 597/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk597(t2476: f64, t4854: f64, t1355: f64, t1367: f64, t2493: f64, t2518: f64, t252: f64, t2530: f64, t2537: f64, t3716: f64, t3754: f64, t4781: f64, t4785: f64, t4817: f64, t4821: f64, t4863: f64, t4869: f64, t4885: f64, t4888: f64, t4897: f64, t4900: f64, t4904: f64, t4920: f64, t810: f64, t829: f64) -> (f64, f64) {
    let t4923 = t4854 * t2476;
    let t4926 = -0.3109e-1_f64 * t4863 * t252 + 2.0_f64 * t3716 * t1355 - 2.0_f64 * t2493 * t4869 + 1.0_f64 * t810 * t4885 + 0.32164683177870697974e2_f64 * t2518 * t4888 + t4897 - t4785 + t4900 - t4817 - t4821 - 0.19751789702565206229e-1_f64 * t4781 + 0.11696446794910408142e1_f64 * t3754 * t1367 - 0.11696446794910408142e1_f64 * t2530 * t4904 + 0.58482233974552040708e0_f64 * t829 * t4920 + 0.17315755899375863299e2_f64 * t2537 * t4923;
    (t4923, t4926)
}
