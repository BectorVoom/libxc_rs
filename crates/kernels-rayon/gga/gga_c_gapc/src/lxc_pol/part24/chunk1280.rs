//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1280/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1280(t35506: f64, t35510: f64, t35512: f64, t35515: f64, t35519: f64, t35521: f64, t35524: f64, t35527: f64, t35531: f64, t35533: f64, t35536: f64, t35539: f64, t35543: f64, t35545: f64) -> f64 {
    let t37414 = 0.50613927761474165061e-5_f64 * t35506 + 0.29524791194193262952e-5_f64 * t35510 - 0.3475929712541504153e-3_f64 * t35512 + 0.21724560703384400956e-4_f64 * t35515 - 0.21724560703384400956e-4_f64 * t35519 + 0.23897016773722841052e-3_f64 * t35521 - 0.21724560703384400956e-4_f64 * t35524 - 0.10862280351692200478e-4_f64 * t35527 - 0.128754229768724883e-5_f64 * t35531 - 0.4004124062907733947e-3_f64 * t35533 + 0.10862280351692200478e-4_f64 * t35536 + 0.128754229768724883e-5_f64 * t35539 + 0.28164987761908568157e-6_f64 * t35543 - 0.10122785552294833012e-4_f64 * t35545;
    t37414
}
