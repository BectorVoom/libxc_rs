//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 820/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk820(t256: f64, t7501: f64, t7342: f64, t7504: f64, t248: f64, t2516: f64, t243: f64, t2520: f64, t7747: f64, t7514: f64, t7517: f64, t7520: f64, t7529: f64, t7538: f64, t7544: f64, t7553: f64, t7555: f64, t7558: f64, t7560: f64, t7563: f64, t7566: f64, t7571: f64, t7573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7753 = t256 * t7501;
    let t7754 = t7342 * t7504;
    let t7758 = 1.0_f64 / t2516 / t248;
    let t7759 = t243 * t7758;
    let t7760 = t7747 * t2520;
    let t7777 = 0.264729375e1_f64 * t7514 - 0.52945875e1_f64 * t7517 + 0.94674375e0_f64 * t7520 + 0.6311625e0_f64 * t7553 + 0.3529725e1_f64 * t7555 - 0.157790625e0_f64 * t7558 - 0.41678000000000000001e0_f64 * t7560 + 0.20839e0_f64 * t7563 - 0.62517e0_f64 * t7566 - 0.103295e1_f64 * t7529 + 0.20659e1_f64 * t7538 - 0.309885e1_f64 * t7544 - 0.34731666666666666667e0_f64 * t7571 + 0.20839e0_f64 * t7573;
    (t7753, t7754, t7758, t7759, t7760, t7777)
}
