//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 768/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk768(t40: f64, t52: f64, t4072: f64, t510: f64, t1774: f64, t671: f64, t1409: f64, t2433: f64, t3966: f64, t607: f64, t73: f64, t2440: f64, t76: f64, t157: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t4073 = t510 * t4072;
    let t4077 = t1774 * t671;
    let t4080 = t2433 * t1409;
    let t4086 = piecewise3(t146, 0.0_f64, 4.0_f64 / 9.0_f64 * t4080 * t607 + 4.0_f64 / 3.0_f64 * t73 * t3966);
    let t4087 = t2440 * t1409;
    let t4093 = piecewise3(t150, 0.0_f64, 4.0_f64 / 9.0_f64 * t4087 * t607 - 4.0_f64 / 3.0_f64 * t76 * t3966);
    let t4094 = t4086 + t4093;
    let t4095 = t4094 * t157;
    (t4073, t4077, t4080, t4087, t4094, t4095)
}
