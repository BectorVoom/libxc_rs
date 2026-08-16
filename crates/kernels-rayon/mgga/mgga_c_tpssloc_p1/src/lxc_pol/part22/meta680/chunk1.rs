//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2244/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2244(t3082: f64, t5905: f64, t10403: f64, t10422: f64, t18035: f64, t17906: f64, t3048: f64, t1041: f64, t248: f64, t43338: f64, t5677: f64, t3070: f64, t43198: f64, t5908: f64) -> (f64, f64, f64, f64, f64) {
    let t62360 = t5905 * t3082;
    let t62418 = t10403 * t10422 * t18035;
    let t62441 = t3048 * t17906;
    let t62445 = t1041 * t248 * t43338 * t5677;
    let t62494 = t3070 * t43198 * t5908;
    (t62360, t62418, t62441, t62445, t62494)
}
