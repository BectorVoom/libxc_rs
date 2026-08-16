//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 947/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk947(t31420: f64, t6547: f64, t23171: f64, t23228: f64, t8547: f64, t31370: f64, t23204: f64, t31419: f64, t6562: f64, t31650: f64, t6883: f64, t31608: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114939 = t6547 * t31420;
    let t114943 = t23171 * t23228 * t8547;
    let t114945 = t6547 * t31370;
    let t114965 = t6562 * t23204 * t31419;
    let t115292 = t6883 * t31650;
    let t115294 = t6883 * t31608;
    (t114939, t114943, t114945, t114965, t115292, t115294)
}
