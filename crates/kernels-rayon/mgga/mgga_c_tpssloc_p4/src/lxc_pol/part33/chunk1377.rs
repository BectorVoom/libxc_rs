//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1377/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1377(t109: f64, t22473: f64, t75603: f64, t20342: f64, t6530: f64, t106944: f64, t81438: f64, t86586: f64, t96713: f64, t96721: f64, t1268: f64, t1458: f64, t5449: f64) -> (f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t106946 = t22473 * t75603;
    let t106948 = t6530 * t20342;
    let t106951 = piecewise3(t110, 0.0_f64, -t81438 - 11.0_f64 / 3.0_f64 * t86586 - 2.0_f64 * t96713 + t96721 - 3.0_f64 / 4.0_f64 * t106944 + 3.0_f64 / 4.0_f64 * t106946 - t106948 / 8.0_f64);
    let t106953 = 2.0_f64 * t1268 * t106951;
    let t106956 = t5449 * t1458;
    (t106951, t106953, t106956)
}
