//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1273/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1273(t247: f64, t24713: f64, t3719: f64, t12900: f64, t17629: f64, t21170: f64, t21189: f64, t21193: f64, t21216: f64, t21234: f64, t21249: f64, t24681: f64, t24684: f64, t24700: f64, t24706: f64, t3718: f64, t484: f64, t5381: f64, t5384: f64, t6683: f64) -> (f64, f64) {
    let t24715 = t247 * t3719 * t24713;
    let t24722 = -0.53100265402527852012e-1_f64 * t24681 * t484 + 0.21722835846488666732e-1_f64 * t24684 * t484 + 0.21437009059034868486e-3_f64 * t24700 * t484 + t21170 / 216.0_f64 - 0.64311027177104605458e-3_f64 * t3718 * t24706 + t12900 + 0.85748036236139473944e-3_f64 * t21189 - 0.85748036236139473944e-3_f64 * t5381 * t6683 - 0.57165357490759649295e-3_f64 * t21193 + 0.12862205435420921092e-2_f64 * t5384 * t24715 - 0.57165357490759649295e-3_f64 * t21216 + t17629 / 432.0_f64 + 0.47637797908966374413e-3_f64 * t21234 + t21249 / 54.0_f64;
    (t24715, t24722)
}
