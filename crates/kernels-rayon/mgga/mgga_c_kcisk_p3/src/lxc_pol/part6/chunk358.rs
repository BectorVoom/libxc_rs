//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 358/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk358(t1471: f64, t1472: f64, t2059: f64, t2209: f64, t416: f64, t140: f64, t1469: f64, t1470: f64, t2221: f64, t2225: f64, t2242: f64, t460: f64, t476: f64, t479: f64) -> (f64, f64, f64) {
    let t2250 = t1471 * t1472 * t2059;
    let t2253 = t416 * t2209;
    let t2257 = 0.619125e-2_f64 * t2242 * t460 + 0.9286875e-2_f64 * t476 * t2221 - 0.619125e-2_f64 * t476 * t2225 - t1469 - 0.26531111111111111111e-1_f64 * t1470 * t2250 - 0.39796666666666666666e-1_f64 * t140 * t479 * t2253;
    (t2250, t2253, t2257)
}
