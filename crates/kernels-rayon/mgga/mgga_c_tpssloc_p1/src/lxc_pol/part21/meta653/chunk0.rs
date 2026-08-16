//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2451/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2451(t11065: f64, t42387: f64, t1005: f64, t10375: f64, t10475: f64, t42342: f64, t42345: f64, t2770: f64, t283: f64, t11064: f64, t42332: f64, t11058: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43361 = t11065 * t42387;
    let t43382 = t1005 * t10375;
    let t43385 = t42342 * t10475 * t42345;
    let t43398 = 1.0_f64 / t283 / t2770;
    let t43470 = t42332 * t11064;
    let t43473 = t42332 * t11058;
    (t43361, t43382, t43385, t43398, t43470, t43473)
}
