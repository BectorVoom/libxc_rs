//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 498/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk498(t123: f64, t250: f64, t132: f64, t721: f64, t759: f64, t762: f64, t791: f64, t256: f64, t729: f64, t257: f64, t2671: f64, t2674: f64, t2677: f64, t2679: f64, t2683: f64, t2685: f64, t2687: f64, t2690: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2723 = t123 * t250;
    let t2736 = t721 * t132 * t759 * t762;
    let t2737 = 0.10685e0_f64 * t2736;
    let t2738 = t132 * t791;
    let t2742 = t729 * t256;
    let t2743 = t2742 * t257;
    let t2754 = -0.47063e1_f64 * t2671 + 0.31375333333333333334e1_f64 * t2674 - 0.36604555555555555556e1_f64 * t2677 - 0.16068111111111111111e1_f64 * t2679 + 0.28051666666666666666e0_f64 * t2683 - 0.56103333333333333332e0_f64 * t2685 - 0.6545388888888888889e0_f64 * t2687 - 0.46308888888888888888e0_f64 * t2690;
    (t2723, t2737, t2738, t2742, t2743, t2754)
}
