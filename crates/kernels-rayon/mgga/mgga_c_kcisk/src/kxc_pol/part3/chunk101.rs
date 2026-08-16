//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 101/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk101(t25: f64, t313: f64, t353: f64, t344: f64, t347: f64, t350: f64, t346: f64, t45: f64, t67: f64, t222: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t355 = t353 * t25 * t313;
    let t357 = 0.379785e1_f64 * t347 + 0.8969e0_f64 * t344 + 0.204775e0_f64 * t350 + 0.24647e0_f64 * t355;
    let t360 = 1.0_f64 + 0.16081824322151104822e2_f64 / t357;
    let t361 = f64::ln(t360);
    let t365 = 1.0_f64 + 0.278125e-1_f64 * t344;
    let t370 = 0.51785e1_f64 * t347 + 0.905775e0_f64 * t344 + 0.1100325e0_f64 * t350 + 0.248355e0_f64 * t355;
    let t373 = 1.0_f64 + 0.29608574643216675549e2_f64 / t370;
    let t374 = f64::ln(t373);
    let t378 = -0.62182e-1_f64 * t346 * t361 + 0.19751789702565206229e-1_f64 * t45 * t365 * t374;
    let t379 = t67 * t378;
    let t380 = t8 * t222;
    let t381 = pow_1_3(t380);
    (t355, t357, t360, t361, t365, t370, t373, t374, t378, t379, t380, t381)
}
