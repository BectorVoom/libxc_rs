//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 728/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk728(t40: f64, t180: f64, t2511: f64, t9489: f64, t9490: f64, t761: f64, t607: f64, t75: f64, t2250: f64, t634: f64, t767: f64, t9258: f64, t9288: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t9493 = 1.0_f64 / t2511 / t180;
    let t9494 = t9489 * t9490 * t9493;
    let t9496 = 0.10254018858216406658e4_f64 * t761 * t9494;
    let t9499 = t75 * t607;
    let t9505 = piecewise3(t146, 0.0_f64, 8.0_f64 / 27.0_f64 * t634 * t9288 - 2.0_f64 / 3.0_f64 * t9499 * t2250 + 2.0_f64 / 3.0_f64 * t767 * t9258);
    (t9493, t9494, t9496, t9505)
}
