//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1527/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1527(t16398: f64, t5252: f64, t3777: f64, t5245: f64, t1834: f64, t3787: f64, t225: f64, t5319: f64, t5217: f64, t1390: f64, t5356: f64, t112: f64, t5363: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16400 = 7.0_f64 / 1152.0_f64 * t16398 * t5252;
    let t16401 = t3777 * t5245;
    let t16428 = t3787 * t1834;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16497 = t5356 * t1390;
    let t16521 = t5363 * t112;
    (t16400, t16401, t16428, t16439, t16460, t16497, t16521)
}
