//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2031/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2031(t2121: f64, t3427: f64, t7295: f64, t11947: f64, t7394: f64, t2157: f64, t43706: f64, t1453: f64, t81439: f64, t26129: f64, t81442: f64, t22470: f64, t4067: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86501 = t2121 * t3427 * t7295;
    let t86517 = t7394 * t11947;
    let t86524 = t2157 * t43706;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    let t86589 = 4.0_f64 / 3.0_f64 * t86588;
    let t86590 = t22470 * t4067;
    (t86501, t86517, t86524, t86586, t86589, t86590)
}
