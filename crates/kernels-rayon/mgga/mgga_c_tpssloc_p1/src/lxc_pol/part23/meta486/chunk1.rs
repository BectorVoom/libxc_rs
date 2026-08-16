//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1491/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1491(t28: f64, t6312: f64, t5966: f64, t19559: f64, t20390: f64, t3672: f64, t39436: f64, t5142: f64, t517: f64, t77953: f64, t157: f64, t79872: f64, t182: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t79873 = t6312 * t6312;
    let t79878 = t5966 * t5966;
    let t79886 = piecewise3(t29, 0.0_f64, 40.0_f64 / 81.0_f64 * t39436 * t79873 - 16.0_f64 / 9.0_f64 * t19559 * t5966 + 4.0_f64 / 3.0_f64 * t3672 * t79878 + 16.0_f64 / 9.0_f64 * t5142 * t20390 + 4.0_f64 / 3.0_f64 * t517 * t77953);
    let t79888 = (t79872 + t79886) * t157;
    let t79890 = 0.19751673498613801407e-1_f64 * t79888 * t182;
    (t79873, t79878, t79888, t79890)
}
