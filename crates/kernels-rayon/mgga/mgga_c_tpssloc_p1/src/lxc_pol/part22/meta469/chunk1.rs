//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1859/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1859(t40: f64, t52: f64, t20217: f64, t20234: f64, t4080: f64, t5398: f64, t73: f64, t9427: f64, t4087: f64, t76: f64, t9438: f64, t157: f64, t182: f64, t16587: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t20732 = piecewise3(t146, 0.0_f64, -8.0_f64 / 27.0_f64 * t9427 * t20234 + 4.0_f64 / 3.0_f64 * t4080 * t5398 + 4.0_f64 / 3.0_f64 * t73 * t20217);
    let t20740 = piecewise3(t150, 0.0_f64, 8.0_f64 / 27.0_f64 * t9438 * t20234 + 4.0_f64 / 3.0_f64 * t4087 * t5398 - 4.0_f64 / 3.0_f64 * t76 * t20217);
    let t20741 = t20732 + t20740;
    let t20742 = t20741 * t157;
    let t20744 = 0.19751673498613801407e-1_f64 * t20742 * t182;
    let t20745 = 36.0_f64 * t16587;
    (t20741, t20742, t20744, t20745)
}
