//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 158/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk158(t606: f64, t25: f64, t353: f64, t579: f64, t609: f64, t45: f64, t608: f64, t67: f64, t227: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t612 = pow_3_2(t606);
    let t615 = t353 * t25 * t579;
    let t617 = 0.379785e1_f64 * t609 + 0.8969e0_f64 * t606 + 0.204775e0_f64 * t612 + 0.24647e0_f64 * t615;
    let t620 = 1.0_f64 + 0.16081824322151104822e2_f64 / t617;
    let t621 = f64::ln(t620);
    let t625 = 1.0_f64 + 0.278125e-1_f64 * t606;
    let t630 = 0.51785e1_f64 * t609 + 0.905775e0_f64 * t606 + 0.1100325e0_f64 * t612 + 0.248355e0_f64 * t615;
    let t633 = 1.0_f64 + 0.29608574643216675549e2_f64 / t630;
    let t634 = f64::ln(t633);
    let t638 = -0.62182e-1_f64 * t608 * t621 + 0.19751789702565206229e-1_f64 * t45 * t625 * t634;
    let t639 = t67 * t638;
    let t640 = t8 * t227;
    let t641 = pow_1_3(t640);
    (t615, t617, t620, t621, t625, t630, t633, t634, t638, t639, t640, t641)
}
