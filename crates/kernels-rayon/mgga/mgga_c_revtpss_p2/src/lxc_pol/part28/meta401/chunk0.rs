//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1512/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1512(t14723: f64, t2662: f64, t2661: f64, t4416: f64, t837: f64, t221: f64, t2485: f64, t4424: f64, t2484: f64, t2652: f64, t4435: f64, t14663: f64, t827: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14724 = t2662 * t14723;
    let t14726 = 0.14291339372689912324e-4_f64 * t2661 * t14724;
    let t14727 = t4416 * t837;
    let t14728 = t2662 * t14727;
    let t14730 = 0.57165357490759649296e-4_f64 * t2661 * t14728;
    let t14732 = t2485 * t221 * t4424;
    let t14734 = 0.25410001404642664112e-4_f64 * t2484 * t14732;
    let t14736 = 0.40015750243531754508e-1_f64 * t2652 * t4435;
    let t14738 = t827 * t828 * t14663;
    (t14726, t14727, t14730, t14732, t14734, t14736, t14738)
}
